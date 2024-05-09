use crate::rpc_helpers::{decode, encode};
use futures::channel::oneshot;
use futures::AsyncRead;
use js_sys::{Array, Function, JsString, Promise, Uint8Array};
use quary_core::databases::DatabaseQueryGenerator;
use quary_core::file_system::FileSystem;
use quary_core::{
    database_bigquery::DatabaseQueryGeneratorBigQuery,
    database_duckdb::DatabaseQueryGeneratorDuckDB,
    database_postgres::DatabaseQueryGeneratorPostgres,
    database_redshift::DatabaseQueryGeneratorRedshift,
    database_snowflake::DatabaseQueryGeneratorSnowflake,
    database_sqlite::DatabaseQueryGeneratorSqlite,
};
use quary_proto::connection_config::Config;
use quary_proto::ConnectionConfig;
use send_wrapper::SendWrapper;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::pin::Pin;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};

pub(crate) type Writer =
    Box<dyn Fn(String, String) -> Pin<Box<dyn Future<Output = Result<(), String>>>>>;

pub(crate) fn create_file_writer(
    file_writer: JsValue,
) -> Box<dyn Fn(String, String) -> Pin<Box<dyn Future<Output = Result<(), String>>>>> {
    let file_writer: Function = file_writer.into();
    let file_writer = move |file_name: String,
                            contents: String|
          -> Pin<Box<dyn Future<Output = Result<(), String>>>> {
        let file_name = JsValue::from_str(&file_name);
        let bytes = contents.into_bytes();
        let contents = Uint8Array::from(&bytes[..]);
        #[allow(clippy::unwrap_used)]
        let promise = file_writer
            .call2(&JsValue::NULL, &file_name, &contents)
            .unwrap()
            .dyn_into::<Promise>()
            .map_err(|err| format!("Failed to convert to promise: {:?}", err))
            .unwrap();
        let promise = JsFuture::from(promise);
        Box::pin(async move {
            promise
                .await
                .map_err(|err| format!("Failed to await js function: {:?}", err))?;
            Ok(())
        })
    };
    Box::new(file_writer)
}

pub(crate) type FileReader =
    Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, io::Error>>>>>;

fn create_file_reader(file_reader: Function) -> FileReader {
    let file_reader =
        move |file_path: String| -> Pin<Box<dyn Future<Output = Result<Vec<u8>, io::Error>>>> {
            let file_path = JsValue::from_str(&file_path);
            #[allow(clippy::unwrap_used)]
            let promise = file_reader
                .call1(&JsValue::NULL, &file_path)
                .unwrap()
                .dyn_into::<Promise>()
                .map_err(|err| format!("Failed to convert to promise: {:?}", err))
                .unwrap();
            let promise = JsFuture::from(promise);
            Box::pin(async move {
                let value = promise.await.map_err(|err| {
                    io::Error::new(
                        ErrorKind::Other,
                        format!("Failed to await js function: {:?}", err),
                    )
                })?;
                let array: Array = value.dyn_into().map_err(|e| {
                    io::Error::new(
                        ErrorKind::Other,
                        format!("Failed to cast output to Array {:?}", e),
                    )
                })?;
                let first_parameter: String = array
                    .at(0)
                    .dyn_into::<JsString>()
                    .map_err(|e| {
                        io::Error::new(
                            ErrorKind::Other,
                            format!("Failed to cast output to JsString {:?}", e),
                        )
                    })?
                    .into();
                match first_parameter.as_str() {
                    "ok" => {
                        let uint8_array = array.at(1).dyn_into::<Uint8Array>().map_err(|e| {
                            io::Error::new(
                                ErrorKind::Other,
                                format!("Failed to cast output to Uint8Array {:?}", e),
                            )
                        })?;
                        Ok(uint8_array.to_vec())
                    }
                    "error" => {
                        let error_message: String = array
                            .at(1)
                            .dyn_into::<JsString>()
                            .map_err(|e| {
                                io::Error::new(
                                    ErrorKind::Other,
                                    format!("Failed to cast output to JsString {:?}", e),
                                )
                            })?
                            .into();
                        Err(io::Error::new(ErrorKind::Other, error_message))
                    }
                    "not_found" => Err(io::Error::new(ErrorKind::NotFound, "File not found")),
                    _ => Err(io::Error::new(
                        ErrorKind::Other,
                        format!("Unknown first parameter type {}", first_parameter),
                    )),
                }
            })
        };
    Box::new(file_reader)
}

pub(crate) type RecursiveFilesLister =
    Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<Vec<String>, String>>>>>;

fn create_recursive_files_lister(js_list_files_recursively: Function) -> RecursiveFilesLister {
    let file_lister =
        move |path: String| -> Pin<Box<dyn Future<Output = Result<Vec<String>, String>>>> {
            let path = JsValue::from_str(&path);
            #[allow(clippy::unwrap_used)]
            let promise = js_list_files_recursively
                .call1(&JsValue::NULL, &path)
                .unwrap()
                .dyn_into::<Promise>()
                .map_err(|err| format!("Failed to convert to promise: {:?}", err))
                .unwrap();
            let promise = JsFuture::from(promise);

            Box::pin(async move {
                let value = promise
                    .await
                    .map_err(|err| format!("Failed to await js function: {:?}", err))?;
                let files_array = value
                    .dyn_into::<Array>()
                    .map_err(|_| "Failed to convert result to array".to_owned())?;
                let files_vec = files_array
                    .iter()
                    .map(|val| val.as_string().unwrap_or_default())
                    .collect::<Vec<String>>();
                Ok(files_vec)
            })
        };

    Box::new(file_lister)
}

pub fn database_query_generator_from_config(
    database: Uint8Array,
) -> Result<Box<dyn DatabaseQueryGenerator>, String> {
    let database = decode::<ConnectionConfig>(database)?;
    let database_config = database
        .config
        .ok_or("config in ConnectionConfig not set")?;

    let database: Box<dyn DatabaseQueryGenerator> = match database_config {
        Config::Sqlite(_) => Box::<DatabaseQueryGeneratorSqlite>::default(),
        Config::SqliteInMemory(_) => Box::<DatabaseQueryGeneratorSqlite>::default(),
        Config::BigQuery(config) => Box::new(DatabaseQueryGeneratorBigQuery::new(
            config.project_id,
            config.dataset_id,
        )),
        Config::Snowflake(config) => Box::new(DatabaseQueryGeneratorSnowflake::new(
            config.database,
            config.schema,
        )),
        Config::Duckdb(config) => Box::new(DatabaseQueryGeneratorDuckDB::new(config.schema, None)),
        Config::DuckdbInMemory(config) => {
            Box::new(DatabaseQueryGeneratorDuckDB::new(config.schema, None))
        }
        Config::Postgres(config) => {
            Box::new(DatabaseQueryGeneratorPostgres::new(config.schema, None))
        }
        Config::Redshift(config) => {
            Box::new(DatabaseQueryGeneratorRedshift::new(config.schema, None))
        }
    };
    Ok(database)
}

pub(crate) fn wrapper<Fut, Req, Res, QueryGenerator>(
    f: impl Fn(QueryGenerator, Writer, JsFileSystem, Req) -> Fut + Clone + 'static,
) -> Box<
    dyn Fn(
        QueryGenerator,
        Writer,
        Function,
        Function,
        Uint8Array,
    ) -> Pin<Box<dyn Future<Output = Result<Uint8Array, String>>>>,
>
where
    Req: prost::Message + Default,
    Res: prost::Message,
    QueryGenerator: DatabaseQueryGenerator + 'static,
    Fut: Future<Output = Result<Res, String>> + 'static,
{
    Box::new(
        move |database: QueryGenerator,
              writer: Writer,
              reader: Function,
              lister: Function,
              request: Uint8Array|
              -> Pin<Box<dyn Future<Output = Result<Uint8Array, String>>>> {
            let js_file_system = JsFileSystem::new(reader, lister);
            let f_clone = f.clone();
            Box::pin(async move {
                let request: Req = decode(request)?;
                let response = f_clone(database, writer, js_file_system, request).await?;
                encode(response)
            })
        },
    )
}

pub(crate) fn wrapper_without_db<Req, Res, Fut>(
    f: impl Fn(Writer, JsFileSystem, Req) -> Fut + Clone + 'static,
) -> Box<
    dyn Fn(
        Writer,
        Function,
        Function,
        Uint8Array,
    ) -> Pin<Box<dyn Future<Output = Result<Uint8Array, String>>>>,
>
where
    Req: prost::Message + Default,
    Res: prost::Message,
    Fut: Future<Output = Result<Res, String>> + 'static,
{
    Box::new(
        move |writer: Writer,
              reader: Function,
              lister: Function,
              request: Uint8Array|
              -> Pin<Box<dyn Future<Output = Result<Uint8Array, String>>>> {
            let js_file_system = JsFileSystem::new(reader, lister);
            let f_clone = f.clone();
            Box::pin(async move {
                let request: Req = decode(request)?;
                let response = f_clone(writer, js_file_system, request).await?;
                encode(response)
            })
        },
    )
}

pub struct JsFileSystem {
    file_reader: SendWrapper<Function>,
    files_lister: SendWrapper<Function>,
}

impl JsFileSystem {
    pub fn new(file_reader: Function, files_lister: Function) -> Self {
        JsFileSystem {
            file_reader: SendWrapper::new(file_reader),
            files_lister: SendWrapper::new(files_lister),
        }
    }
}

#[async_trait::async_trait]
impl FileSystem for JsFileSystem {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error> {
        let file_reader = self.file_reader.clone();
        let path = path.to_string();

        let (sender, receiver) = oneshot::channel::<Result<Vec<u8>, io::Error>>();

        spawn_local(async move {
            // Perform some async computation
            let result = async move {
                let file_reader = file_reader.deref().clone();
                let file_reader = create_file_reader(file_reader);
                file_reader(path).await
            }
            .await;

            let _ = sender.send(result);
        });

        let vec = receiver
            .await
            .map_err(|e| {
                io::Error::new(
                    ErrorKind::Other,
                    format!("Failed to receive result: {:?}", e),
                )
            })
            .map_err(|e| {
                io::Error::new(
                    ErrorKind::Other,
                    format!("Failed to receive result: {:?}", e),
                )
            })??;

        Ok(Box::new(futures::io::Cursor::new(vec)))
    }

    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        let file_lister = self.files_lister.clone();

        let (sender, receiver) = oneshot::channel::<Result<Vec<String>, String>>();
        let path = path.to_string();

        spawn_local(async move {
            // Perform some async computation
            let result = async move {
                let file_lister = file_lister.deref().clone();
                let file_lister = create_recursive_files_lister(file_lister);
                file_lister(path).await
            }
            .await;

            // Send the result through the channel
            let _ = sender.send(result);
        });

        receiver
            .await
            .map_err(|e| format!("Failed to receive result: {:?}", e))?
    }
}
