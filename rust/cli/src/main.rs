#![deny(clippy::expect_used)]
#![deny(clippy::needless_lifetimes)]
#![deny(clippy::needless_borrow)]
#![deny(clippy::useless_conversion)]
#![deny(clippy::unwrap_used)]
#![deny(unused_imports)]
#![deny(unused_import_braces)]

use crate::commands::{Cli, Commands, InitType};
use crate::file_system::LocalFS;
use crate::rpc_functions::rpc;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use quary_core::automatic_branching::{
    derive_hash_views, drop_statement_for_cache_view, is_cache_full_path,
};
use quary_core::config::deserialize_config_from_yaml;
use quary_core::databases::{ColumnWithDetails, DatabaseConnection, DatabaseQueryGenerator};
use quary_core::graph::project_to_graph;
use quary_core::init::{Asset, DuckDBAsset};
use quary_core::onboarding::is_empty_bar_hidden_and_sqlite;
use quary_core::project_tests::return_tests_sql;
use quary_core::project_to_sql::{
    project_and_fs_to_sql_for_snapshots, project_and_fs_to_sql_for_views,
};
use quary_core::test_runner::{
    run_tests_internal, RunReturnResult, RunStatementFunc, RunTestError,
};
use quary_databases::databases_connection::{
    database_from_config, database_query_generator_from_config,
};
use quary_proto::test_result::TestResult;
use quary_proto::{
    failed, passed, ColumnTest, ConnectionConfig, ProjectFile, ProjectFileColumn,
    ProjectFileSource, TableAddress,
};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

mod commands;
mod commands_experimental;
mod file_system;
mod rpc_functions;
mod rpc_scaffolding;

// TODO For the cases where don't need full database, separate that out in the future
#[tokio::main]
async fn main() {
    match main_wrapped().await {
        Ok(_) => {}
        Err(e) => {
            // replace \\n with \n
            let out = e.replace("\\n", "\n");
            eprint!("{}", out);
            std::process::exit(1);
        }
    }
}

async fn main_wrapped() -> Result<(), String> {
    let args = Cli::parse();

    match &args.env_files[..] {
        files if files == [".env"] => {
            dotenv::dotenv().ok();
        }
        _ => {
            for env_file in &args.env_files {
                if dotenv::from_filename(env_file).is_err() {
                    return Err(format!("Error loading env file: {}", env_file));
                }
            }
        }
    }

    match &args.command {
        Commands::Init(args) => {
            let dir = std::env::current_dir().map_err(|e| e.to_string())?;
            let fs = LocalFS::new(dir);

            if !is_empty_bar_hidden_and_sqlite(&fs, ".")
                .await
                .map_err(|e| e.to_string())?
            {
                return Err("Directory is not empty".to_string());
            }

            match args.mode {
                InitType::Sqlite => {
                    for file in Asset::iter() {
                        let filename = file.as_ref();
                        let path = Path::new(filename);
                        let prefix = path
                            .parent()
                            .ok_or("Could not get parent directory for file in Asset::iter()")?;
                        if !prefix.exists() {
                            fs::create_dir_all(prefix).map_err(|e| e.to_string())?;
                        }

                        if let Some(content) = Asset::get(filename) {
                            let mut output =
                                File::create(Path::new(filename)).map_err(|e| e.to_string())?;
                            output.write_all(&content.data).map_err(|e| e.to_string())?;
                        }
                    }
                    Ok(())
                }
                InitType::Duckdb => {
                    for file in DuckDBAsset::iter() {
                        let filename = file.as_ref();
                        let path = Path::new(filename);
                        let prefix = path.parent().ok_or(
                            "Could not get parent directory for file in DuckDBAsset::iter()",
                        )?;
                        if !prefix.exists() {
                            fs::create_dir_all(prefix).map_err(|e| e.to_string())?;
                        }

                        if let Some(content) = DuckDBAsset::get(filename) {
                            let mut output =
                                File::create(Path::new(filename)).map_err(|e| e.to_string())?;
                            output.write_all(&content.data).map_err(|e| e.to_string())?;
                        }
                    }
                    Ok(())
                }
            }
        }
        Commands::Compile => {
            println!("Starting compilation process...");

            let config = get_config_file(&args.project_file)
                .map_err(|e| format!("Error getting config: {}", e))?;

            let database = database_query_generator_from_config(config)
                .map_err(|e| format!("Error creating database query generator: {}", e))?;

            let (project, _) = parse_project(&database)
                .await
                .map_err(|e| format!("Error parsing project: {}", e))?;

            println!("Models processed: {}", project.models.len());
            println!("Tests processed: {}", project.tests.len());
            println!("Snapshots processed: {}", project.snapshots.len());
            println!("Charts processed: {}", project.charts.len());
            println!("Project compiled successfully.");
            Ok(())
        }
        Commands::Build(build_args) => {
            let config = get_config_file(&args.project_file)?;

            let database = database_from_config(&config).await?;
            let query_generator = database.query_generator();
            let (project, file_system) = parse_project(&query_generator).await?;

            // If cache table deletes any previous cache views.
            let cache_delete_views_sqls: Vec<(String, String)> = if build_args.cache_views {
                let views = database
                    .list_local_views()
                    .await
                    .map_err(|e| format!("listing views: {:?}", e))?;
                let views_with_is_cache = views
                    .into_iter()
                    .map(|view| {
                        let is_cache =
                            is_cache_full_path(&database.query_generator(), &view.full_path)?;
                        Ok((view, is_cache))
                    })
                    .collect::<Result<Vec<_>, String>>()?;

                let filtered_views = views_with_is_cache
                    .into_iter()
                    .filter_map(|(view, is_cache)| if is_cache { Some(view) } else { None })
                    .collect::<Vec<_>>();
                let views_with_delete_statements: Vec<(String, String)> = filtered_views
                    .into_iter()
                    .map(|view| {
                        let delete_statement = drop_statement_for_cache_view(&view.full_path)?;
                        Ok((view.name, delete_statement))
                    })
                    .collect::<Result<Vec<(String, String)>, String>>()?;

                Ok::<_, String>(views_with_delete_statements)
            } else {
                Ok(vec![])
            }?;

            let cache_to_create: Vec<(String, Vec<String>)> = if build_args.cache_views {
                let project_graph = project_to_graph(project.clone())?;
                let views =
                    derive_hash_views(&database.query_generator(), &project, &project_graph)?;
                Ok::<_, String>(
                    views
                        .into_iter()
                        .map(|(name, (_, sql))| (name.to_string(), sql))
                        .collect(),
                )
            } else {
                Ok(vec![])
            }?;

            let sqls = project_and_fs_to_sql_for_views(
                &project,
                &file_system,
                &query_generator,
                false,
                false,
                true,
            )
            .await?;

            if build_args.dry_run {
                if !cache_delete_views_sqls.is_empty() {
                    println!("\n-- Delete cache views\n");
                    for (name, sql) in cache_delete_views_sqls {
                        println!("-- {}", name);
                        println!("{};", sql);
                    }
                }
                println!("\n-- Create models\n");
                for (name, sql) in sqls {
                    println!("\n-- {name}");
                    for sql in sql {
                        println!("{};", sql);
                    }
                }
                if !cache_to_create.is_empty() {
                    println!("\n-- Create cache views\n");
                    for (name, sql) in &cache_to_create {
                        println!("-- {}", name);
                        for sql in sql {
                            println!("{};", sql);
                        }
                    }
                }
                Ok(())
            } else {
                let total_number_of_sql_statements = cache_delete_views_sqls.len()
                    + sqls.iter().map(|(_, sqls)| sqls.len()).sum::<usize>()
                    + cache_to_create.len();
                let pb = ProgressBar::new(total_number_of_sql_statements as u64);
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                        .map_err(|e| e.to_string())?
                        .progress_chars("=>-"),
                );
                for (name, sql) in cache_delete_views_sqls {
                    pb.set_message(format!("Building model: {}", name));
                    pb.inc(1);
                    database.exec(sql.as_str()).await.map_err(|e| {
                        format!("executing sql for model '{}': {:?} {:?}", name, sql, e)
                    })?
                }
                for (name, sql) in &sqls {
                    for sql in sql {
                        pb.set_message(format!("Building model: {}", name));
                        pb.inc(1);
                        database.exec(sql.as_str()).await.map_err(|e| {
                            format!("executing sql for model '{}': {:?} {:?}", name, sql, e)
                        })?
                    }
                }
                for (name, sql) in &cache_to_create {
                    pb.inc(1);
                    pb.set_message(name.to_string());
                    for sql in sql {
                        database.exec(sql.as_str()).await.map_err(|e| {
                            format!("executing sql for model '{}': {:?} {:?}", name, sql, e)
                        })?
                    }
                }
                pb.finish_with_message("done");
                match sqls.len() {
                    0 => println!("No models to build"),
                    1 => println!("Created 1 model in the database"),
                    n => println!("Created {} models in the database", n),
                }

                if build_args.cache_views {
                    match cache_to_create.len() {
                        0 => println!("No cache views to build"),
                        1 => println!("Created 1 cache view in the database"),
                        n => println!("Created {} cache views in the database", n),
                    }
                }
                Ok(())
            }
        }
        Commands::Test(test_args) => {
            let config = get_config_file(&args.project_file)?;

            let database = database_from_config(&config).await?;
            let query_generator = database.query_generator();
            let (project, file_system) = parse_project(&query_generator).await?;

            let limit = if test_args.verbose { None } else { Some(1) };

            let tests = return_tests_sql(
                &database.query_generator(),
                &project,
                &file_system,
                test_args.full_source,
                limit,
                None,
            )
            .await?;

            if test_args.dry_run {
                for (name, test) in tests {
                    println!("-- {name}\n{test};\n");
                }
                return Ok(());
            }

            let mode = test_args.mode.to_test_runner();
            let database = database_from_config(&config)
                .await
                .map_err(|e| format!("reading database from config: {:?}", e))?;
            println!("{:?}", database);

            let database = Arc::new(database);
            let func: RunStatementFunc = Box::new(move |sql: &str| {
                let database = Arc::clone(&database);
                let sql = sql.to_owned();

                Box::pin(async move {
                    let result = database.query(&sql).await;
                    match result {
                        Ok(outs) => Ok(if outs.rows.is_empty() {
                            RunReturnResult::Passed
                        } else {
                            let proto = outs.to_proto()?;
                            RunReturnResult::QueryResult(proto)
                        }),
                        Err(error) => Err(format!("Error running query: {:?}", error)),
                    }
                })
            });

            // TODO Need to reintroduce the progressbar
            println!("running tests {}", tests.len());
            let dialect = query_generator.get_dialect();
            let parser = sqruff_lib_core::parser::parser::Parser::new(&dialect, Default::default());
            return match run_tests_internal(
                &query_generator,
                &file_system,
                &project,
                &parser,
                mode,
                func,
                test_args.full_source,
                limit,
            )
            .await
            {
                Err(e) => match e {
                    RunTestError::Other(error) => Err(error),
                    RunTestError::TestFailedToRun(error) => Err(format!(
                        "Test '{}' failed to run: {} with sql '{}'",
                        error.test_name, error.error, error.sql
                    )),
                },
                Ok(tests) => {
                    let tests_pass = tests
                        .results
                        .iter()
                        .filter(|r| matches!(r.test_result, Some(TestResult::Passed(_))))
                        .count();
                    let tests_fail: Vec<_> = tests
                        .results
                        .iter()
                        .filter(|r| matches!(r.test_result, Some(TestResult::Failed(_))))
                        .collect();
                    let tests_inferred = tests
                        .results
                        .iter()
                        .filter(|r| match &r.test_result {
                            Some(result) => match result {
                                TestResult::Passed(reason) => match &reason.reason {
                                    Some(reason) => match reason {
                                        passed::Reason::InferredFromTests(_) => true,
                                        passed::Reason::InferredFromLogic(_) => true,
                                        passed::Reason::InferredThroughTestsOperation(_) => true,
                                        passed::Reason::Ran(_) => false,
                                    },
                                    None => false,
                                },
                                TestResult::Failed(reason) => match &reason.reason {
                                    Some(reason) => match reason {
                                        failed::Reason::InferredFromTests(_) => true,
                                        failed::Reason::InferredThroughTestsOperation(_) => true,
                                        failed::Reason::Ran(_) => false,
                                        failed::Reason::FailedRunMessage(_) => false,
                                    },
                                    None => false,
                                },
                            },
                            None => false,
                        })
                        .count();
                    if tests_fail.is_empty() {
                        if tests_inferred == 0 {
                            println!("all tests ({}) were run and passed.", tests_pass);
                            Ok(())
                        } else {
                            println!(
                                "all tests tests ({}) passed, {} tests were run, {} were inferred ",
                                tests_pass,
                                tests_pass - tests_inferred,
                                tests_inferred
                            );
                            Ok(())
                        }
                    } else {
                        println!(
                            "{} tests passed, {} tests failed",
                            tests_pass,
                            tests_fail.len()
                        );
                        println!("failed tests:");
                        for test in &tests_fail {
                            println!("  {}", test.test_name);
                            if test_args.verbose {
                                if let Some(TestResult::Failed(reason)) = &test.test_result {
                                    if let Some(reason) = &reason.reason {
                                        match reason {
                                            failed::Reason::FailedRunMessage(message) => {
                                                println!(
                                                    "    failed run message: {}",
                                                    message.message
                                                )
                                            }
                                            failed::Reason::InferredFromTests(_) => {
                                                println!("    inferred from tests")
                                            }
                                            failed::Reason::InferredThroughTestsOperation(_) => {
                                                println!("    inferred through test operation")
                                            }
                                            failed::Reason::Ran(results) => {
                                                println!("    ran and returned results");
                                                while let Some(result) = &results.query_result {
                                                    println!("    {:?}", result.columns);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(format!("{} tests failed", tests_fail.len()))
                    }
                }
            };
        }
        Commands::ConvertDbt(dbt_args) => {
            let start = Instant::now();
            let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
            let target_dir = current_dir.join(&dbt_args.quary_project_path);

            // if target dir doesn't exist, create it
            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
            }

            match dbt_converter::project::parse_dbt_project(&current_dir, &target_dir) {
                Ok(stats) => {
                    println!("Succesfully converted DBT -> Quary ✅");
                    println!("📦 Models converted: {}", stats.models_created);
                    println!("🧪 Tests converted: {}", stats.tests_created);
                    println!("🧪 Test files converted: {}", stats.file_tests_created);
                    println!("🌱 Seeds transferred: {}", stats.seeds_created);
                    println!("⚠️ Warnings:");
                    for message in &stats.errors {
                        println!("{}", message);
                    }
                    let duration = start.elapsed();
                    println!("⏱️ Time taken: {:.2?} milliseconds", duration.as_millis());
                }
                Err(e) => {
                    println!("Error converting dbt project: {}", e);
                }
            }
            Ok(())
        }
        Commands::GenerateSources(_) => {
            let config = get_config_file(&args.project_file)?;
            let database = database_from_config(&config).await?;
            let sources = generate_sources(database.as_ref(), &Some("raw_")).await?;
            let project_file = ProjectFile {
                sources,
                models: vec![],
                snapshots: vec![],
            };
            let yaml = serde_yaml::to_string(&project_file).map_err(|e| e.to_string())?;
            print!("{}", yaml);
            Ok(())
        }
        Commands::Rpc(rpc_args) => rpc(&args, rpc_args).await,
        Commands::Snapshot(snapshot_args) => {
            let config = get_config_file(&args.project_file)?;
            let database = database_from_config(&config).await?;
            let query_generator = database.query_generator();
            let (project, file_system) = parse_project(&query_generator).await?;
            let snapshots_sql = project_and_fs_to_sql_for_snapshots(
                &project,
                &file_system,
                &query_generator,
                database.as_ref(),
            )
            .await?;

            if snapshot_args.dry_run {
                println!("\n-- Create snapshots\n");
                for (name, sql) in snapshots_sql {
                    println!("\n-- {name}");
                    for statement in sql {
                        println!("{};", statement);
                    }
                }
                Ok(())
            } else {
                let total_number_of_snapshots = project.snapshots.len();
                let pb = ProgressBar::new(total_number_of_snapshots as u64);
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("{spinner:.green} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                        .map_err(|e| e.to_string())?
                        .progress_chars("=>-"),
                );

                for (name, sql) in snapshots_sql {
                    pb.set_message(format!("Building snapshot: {}", name));
                    pb.inc(1);
                    for statement in sql {
                        database.exec(statement.as_str()).await.map_err(|e| {
                            format!(
                                "executing sql for snapshot '{}': {:?} {:?}",
                                name, statement, e
                            )
                        })?;
                    }
                }

                pb.finish_with_message("done");
                println!(
                    "Created/updated {} snapshot(s) in the database",
                    total_number_of_snapshots
                );
                Ok(())
            }
        }
        Commands::Experimental(experimental_args) => {
            crate::commands_experimental::experimental_commands(
                experimental_args,
                &args.project_file,
            )
            .await
        }
    }
}

async fn generate_sources(
    database: &dyn DatabaseConnection,
    source_name_prefix: &Option<&str>,
) -> Result<Vec<ProjectFileSource>, String> {
    let tables = database.list_tables().await?;
    let views = database.list_views().await?;

    let mut tables_with_columns = vec![];
    for table in tables.into_iter().chain(views.into_iter()) {
        let columns = database.list_columns(&table.full_path).await?;
        tables_with_columns.push(AddressWithColumns { table, columns });
    }

    Ok(tables_with_columns
        .into_iter()
        .map(|address_with_columns| address_to_source(address_with_columns, source_name_prefix))
        .collect())
}

struct AddressWithColumns {
    table: TableAddress,
    columns: Vec<ColumnWithDetails>,
}

fn address_to_source(
    address_with_columns: AddressWithColumns,
    source_name_prefix: &Option<&str>,
) -> ProjectFileSource {
    let source_name = match source_name_prefix {
        Some(prefix) => format!("{}{}", prefix, address_with_columns.table.name),
        None => address_with_columns.table.name,
    };
    ProjectFileSource {
        name: source_name,
        tags: vec![],
        description: None,
        path: address_with_columns.table.full_path,
        tests: vec![],
        columns: address_with_columns
            .columns
            .into_iter()
            .map(|column| ProjectFileColumn {
                name: column.name,
                description: column.description,
                tests: vec![
                    if column.is_nullable.unwrap_or(false) {
                        Some(ColumnTest {
                            r#type: "not_null".to_string(),
                            info: Default::default(),
                        })
                    } else {
                        None
                    },
                    if column.is_unique.unwrap_or(false) {
                        Some(ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        })
                    } else {
                        None
                    },
                ]
                .into_iter()
                .flatten()
                .collect(),
            })
            .collect(),
    }
}

async fn parse_project(
    database: &impl DatabaseQueryGenerator,
) -> Result<(quary_proto::Project, LocalFS), String> {
    let dir = std::env::current_dir().map_err(|e| e.to_string())?;
    let filesystem = LocalFS::new(dir);
    let project = quary_core::project::parse_project(&filesystem, database, "").await?;
    Ok((project, filesystem))
}

// TODO this should be replaced with get_config_from_filesystem
fn get_config_file(cfg_file: &str) -> Result<ConnectionConfig, String> {
    let file = File::open(format!("./{}", cfg_file))
        .map_err(|e| format!("opening config file {:?}: {:?}", cfg_file, e))?;
    let config = deserialize_config_from_yaml(file)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_to_source() {
        let address = AddressWithColumns {
            table: TableAddress {
                name: "table".to_string(),
                full_path: "schema.table".to_string(),
            },
            columns: vec![
                ColumnWithDetails {
                    name: "column1".to_string(),
                    description: None,
                    data_type: None,
                    is_nullable: None,
                    is_unique: None,
                },
                ColumnWithDetails {
                    name: "column2".to_string(),
                    description: Some("description 1".to_string()),
                    data_type: Some("data_type".to_string()),
                    is_nullable: Some(false),
                    is_unique: Some(false),
                },
                ColumnWithDetails {
                    name: "column3".to_string(),
                    description: Some("description 2".to_string()),
                    data_type: Some("data_type".to_string()),
                    is_nullable: Some(true),
                    is_unique: Some(true),
                },
            ],
        };
        let source = address_to_source(address, &Some("raw_"));

        assert_eq!(source.name, "raw_table");
        assert_eq!(source.tags.len(), 0);
        assert_eq!(source.description, None);
        assert_eq!(source.path, "schema.table");
        assert_eq!(source.tests, vec![]);
        assert_eq!(source.columns.len(), 3);
        assert_eq!(
            source.columns,
            vec![
                ProjectFileColumn {
                    name: "column1".to_string(),
                    description: None,
                    tests: vec![],
                },
                ProjectFileColumn {
                    name: "column2".to_string(),
                    description: Some("description 1".to_string()),
                    tests: vec![],
                },
                ProjectFileColumn {
                    name: "column3".to_string(),
                    description: Some("description 2".to_string()),
                    tests: vec![
                        ColumnTest {
                            r#type: "not_null".to_string(),
                            info: Default::default(),
                        },
                        ColumnTest {
                            r#type: "unique".to_string(),
                            info: Default::default(),
                        },
                    ],
                },
            ]
        );
    }
}
