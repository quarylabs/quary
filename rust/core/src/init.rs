use crate::file_system::FileSystem;
use futures::io::Cursor;
use futures::AsyncRead;
use quary_proto::{File, FileSystem as ProtoFileSystem};
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io;

#[derive(RustEmbed)]
#[folder = "./src/init"]
pub struct Asset;

#[derive(RustEmbed)]
#[folder = "./src/init_duckdb"]
pub struct DuckDBAsset;

#[async_trait::async_trait]
impl FileSystem for Asset {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error> {
        match Asset::get(path) {
            Some(file) => Ok(Box::new(Cursor::new(file.data.into_owned()))),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Could not find file at path: {}", path),
            )),
        }
    }

    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        let files = Asset::iter()
            .filter_map(|file| {
                let filename = file.as_ref();
                if filename.starts_with(path) {
                    Some(filename.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        Ok(files)
    }
}

#[async_trait::async_trait]
impl FileSystem for DuckDBAsset {
    async fn read_file(&self, path: &str) -> Result<Box<dyn AsyncRead + Send + Unpin>, io::Error> {
        match DuckDBAsset::get(path) {
            Some(file) => Ok(Box::new(Cursor::new(file.data.into_owned()))),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Could not find file at path: {}", path),
            )),
        }
    }

    async fn list_all_files_recursively(&self, path: &str) -> Result<Vec<String>, String> {
        let files = DuckDBAsset::iter()
            .filter_map(|file| {
                let filename = file.as_ref();
                if filename.starts_with(path) {
                    Some(filename.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        Ok(files)
    }
}

pub fn init_to_file_system() -> ProtoFileSystem {
    let file_map: HashMap<String, File> = Asset::iter().fold(HashMap::new(), |mut acc, file| {
        let filename = file.as_ref();
        if let Some(asset) = Asset::get(filename) {
            let data = Cow::to_owned(&asset.data);
            acc.insert(
                filename.to_string(),
                File {
                    name: filename.to_string(),
                    contents: prost::bytes::Bytes::from(data.into_owned()),
                },
            );
            acc
        } else {
            panic!("Could not find file at path: {}", filename);
        }
    });
    ProtoFileSystem { files: file_map }
}

pub const INIT_FOLDER_NUMBER_OF_TESTS: usize = 66;
pub const INIT_FOLDER_NUMBER_OF_TESTS_THAT_FAIL: usize = 0;
pub const INIT_FOLDER_NUMBER_OF_TESTS_THAT_ARE_RUN: usize = 44;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;
    use crate::project::parse_project;

    #[test]
    fn test_init_to_file_system() {
        let fs = init_to_file_system();

        assert!(!fs.files.is_empty());
    }

    #[tokio::test]
    async fn test_init_parse_project() {
        let db = DatabaseQueryGeneratorSqlite::default();

        let fs = init_to_file_system();

        let project = parse_project(&fs, &db, "").await.unwrap();

        assert!(!project.models.is_empty());
        assert!(!project.tests.is_empty());
        assert_eq!(project.project_files.len(), 2);
    }

    //     TODO Impleemnt
    //     // TestInit_ParseProjectAndApply tests that the project can be parsed and applied to a database. This test works fully in
    // // memory.
    // func TestInit_ParseProjectAndApply(t *testing.T) {
    // 	t.Parallel()
    //
    // 	ctx := context.Background()
    //
    // 	f := lib.Init()
    //
    // 	configBytes, err := fs.ReadFile(f, "init/project.yml")
    // 	require.NoError(t, err)
    //
    // 	reader := bytes.NewReader(configBytes)
    // 	c, err := lib.ParseConfig(reader)
    // 	require.NoError(t, err)
    //
    // 	p, err := lib.ParseProject(c, f, "init/")
    // 	require.NoError(t, err)
    //
    // 	db, err := databasesImplementation.NewSqlLiteInMemory()
    // 	require.NoError(t, err)
    //
    // 	// Run the project
    // 	{
    // 		sqls, err := lib.ProjectAndFsToSqlForViews(p, f, db, false, false)
    // 		require.NoError(t, err)
    // 		require.Greater(t, len(sqls), 0)
    // 		for _, sql := range sqls {
    // 			_, err := db.ExecContext(ctx, sql[1])
    // 			require.NoError(t, err, fmt.Sprintf("sql: %s", sql))
    // 		}
    // 	}
    //
    // 	// Run the tests
    // 	{
    // 		tests, err := lib.ReturnTestsSQL(p, f)
    // 		require.NoError(t, err)
    // 		require.Greater(t, len(tests), 0)
    // 		for name, test := range tests {
    // 			err := lib.RunTestSql(ctx, db, test)
    // 			require.NoError(t, err, fmt.Sprintf("test name: %s \nsql: %s", name, test))
    // 		}
    // 	}
    // }
    //
    // // TestInit_ParseProjectAndApply_WithSources does what TestInit_ParseProjectAndApply does, but it also tests that the
    // // sources are applied correctly. It does so by adding to the init project:
    // // - Preloaded table in the database called pay_band_table
    // // - A source that references the pay-band table into the database (where the reference and the name of the model are different)
    // // - A model that references the source
    // //
    // // - In addition it also adds a table to the database called employees_with_pay_bands_references
    // // - A source that references the pay-band source
    // // - The `pay_band_table` source does a foreign key reference test against it
    // func TestInit_ParseProjectAndApply_WithSources(t *testing.T) {
    // 	t.Parallel()
    //
    // 	ctx := context.Background()
    //
    // 	f := lib.Init()
    // 	memFS, err := copyToMapFS(f, "init")
    // 	require.NoError(t, err)
    //
    // 	db, err := databasesImplementation.NewSqlLiteInMemory()
    // 	require.NoError(t, err)
    //
    // 	// Add the pay-band table to the database
    // 	{
    // 		_, err := db.ExecContext(ctx, "CREATE TABLE pay_band_table (id INTEGER PRIMARY KEY, name TEXT, min INTEGER, max INTEGER)")
    // 		require.NoError(t, err)
    // 		// TODO Insert data
    // 		_, err = db.ExecContext(ctx, `
    // INSERT INTO pay_band_table (id, name, min, max) VALUES
    // (1, 'Band 1', 0, 10000),
    // (2, 'Band 2', 10001, 20000);
    // `)
    // 		require.NoError(t, err)
    // 	}
    // 	// Add the employees_with_pay_bands_references table
    // 	{
    // 		_, err := db.ExecContext(ctx, "CREATE TABLE employees_with_pay_bands_references (id INTEGER PRIMARY KEY, pay_band_id INTEGER)")
    // 		require.NoError(t, err)
    // 		_, err = db.ExecContext(ctx, `
    // INSERT INTO employees_with_pay_bands_references (id, pay_band_id) VALUES
    // (1, 1),
    // (2, 2);
    // `)
    // 		require.NoError(t, err)
    //
    // 	}
    // 	// Add the employees_with_pay_bands_references source
    // 	{
    // 		memFS["models/pay-band-2.yml"] = &fstest.MapFile{
    // 			Data: []byte(`
    // sources:
    //   - name: pay_band_references
    //     path: employees_with_pay_bands_references
    //     columns:
    //       - name: id
    //         tests:
    //           - type: not_null
    //           - type: unique
    //       - name: key_band_id
    // `),
    // 		}
    // 	}
    // 	// Add model reference in sql
    // 	memFS["models/employees_with_pay_bands.sql"] = &fstest.MapFile{
    // 		Data: []byte(`SELECT id FROM q.pay_band`),
    // 	}
    // 	// Add reference in project file
    // 	memFS["models/pay-band.yml"] = &fstest.MapFile{
    // 		Data: []byte(`
    // sources:
    //   - name: pay_band
    //     path: pay_band_table
    //     columns:
    //       - name: id
    //         tests:
    //           - type: not_null
    //           - type: unique
    //           - type: relationship
    //             info:
    //               model: pay_band_references
    //               column: pay_band_id
    //   - name: name
    //   - name: min
    //   - name: max
    //
    // models:
    //   - name: employees_with_pay_bands
    //     columns:
    //       - name: id
    //         tests:
    //           - type: not_null
    //           - type: unique
    // `),
    // 	}
    //
    // 	// Parse the project
    // 	configBytes, err := fs.ReadFile(memFS, "project.yml")
    // 	require.NoError(t, err)
    //
    // 	reader := bytes.NewReader(configBytes)
    // 	c, err := lib.ParseConfig(reader)
    // 	require.NoError(t, err)
    //
    // 	p, err := lib.ParseProject(c, memFS, "")
    // 	require.NoError(t, err)
    //
    // 	// Run the project
    // 	{
    // 		sqls, err := lib.ProjectAndFsToSqlForViews(p, memFS, db, false, false)
    // 		require.NoError(t, err)
    // 		require.Greater(t, len(sqls), 0)
    // 		for _, sql := range sqls {
    // 			_, err := db.ExecContext(ctx, sql[1])
    // 			require.NoError(t, err, fmt.Sprintf("sql: %s", sql))
    // 		}
    // 	}
    //
    // 	// Run the tests
    // 	{
    // 		tests, err := lib.ReturnTestsSQL(p, memFS)
    // 		require.NoError(t, err)
    // 		require.Greater(t, len(tests), 0)
    // 		for name, test := range tests {
    // 			err := lib.RunTestSql(ctx, db, test)
    // 			require.NoError(t, err, fmt.Sprintf("test name: %s \nsql: %s", name, test))
    // 		}
    // 	}
    // }
    //
    // func copyToMapFS(sourceFS fs.FS, root string) (fstest.MapFS, error) {
    // 	mapFS := fstest.MapFS{}
    //
    // 	err := fs.WalkDir(sourceFS, root, func(path string, d fs.DirEntry, err error) error {
    // 		if err != nil {
    // 			return err
    // 		}
    //
    // 		if !d.IsDir() {
    // 			srcFile, err := sourceFS.Open(path)
    // 			if err != nil {
    // 				return err
    // 			}
    // 			defer srcFile.Close()
    //
    // 			var content []byte
    // 			content, err = io.ReadAll(srcFile)
    // 			if err != nil {
    // 				return err
    // 			}
    //
    // 			// Remove the root prefix from the path
    // 			relPath, err := filepath.Rel(root, path)
    // 			if err != nil {
    // 				return err
    // 			}
    //
    // 			mapFS[relPath] = &fstest.MapFile{
    // 				Data: content,
    // 				Mode: d.Type(),
    // 			}
    // 		}
    // 		return nil
    // 	})
    // 	if err != nil {
    // 		return nil, fmt.Errorf("failed to copy files: %w", err)
    // 	}
    //
    // 	return mapFS, nil
    // }
    //
    // func TestInit_ParseProjectAndApplySchemasOnly(t *testing.T) {
    // 	t.Parallel()
    //
    // 	ctx := context.Background()
    //
    // 	f := lib.Init()
    //
    // 	configBytes, err := fs.ReadFile(f, "init/project.yml")
    // 	require.NoError(t, err)
    //
    // 	reader := bytes.NewReader(configBytes)
    // 	c, err := lib.ParseConfig(reader)
    // 	require.NoError(t, err)
    //
    // 	p, err := lib.ParseProject(c, f, "init/")
    // 	require.NoError(t, err)
    //
    // 	db, err := databasesImplementation.NewSqlLiteInMemory()
    // 	require.NoError(t, err)
    //
    // 	// Run the project
    // 	{
    // 		sqls, err := lib.ProjectAndFsToSqlForViews(p, f, db, false, true)
    // 		require.NoError(t, err)
    // 		require.Greater(t, len(sqls), 0)
    // 		for _, sql := range sqls {
    // 			_, err := db.ExecContext(ctx, sql[1])
    // 			require.NoError(t, err, fmt.Sprintf("sql: %s", sql))
    // 		}
    // 	}
    //
    // 	// Run the tests
    // 	{
    // 		tests, err := lib.ReturnTestsSQL(p, f)
    // 		require.NoError(t, err)
    // 		require.Greater(t, len(tests), 0)
    // 		for name, test := range tests {
    // 			err := lib.RunTestSql(ctx, db, test)
    // 			require.NoError(t, err, fmt.Sprintf("test name: %s \nsql: %s", name, test))
    // 		}
    // 	}
    // }
    //
    // func TestInit_SelectStatementTest(t *testing.T) {
    // 	t.Parallel()
    //
    // 	ctx := context.Background()
    //
    // 	f := lib.Init()
    // 	configBytes, err := fs.ReadFile(f, "init/project.yml")
    // 	require.NoError(t, err)
    //
    // 	reader := bytes.NewReader(configBytes)
    // 	c, err := lib.ParseConfig(reader)
    // 	require.NoError(t, err)
    //
    // 	p, err := lib.ParseProject(c, f, "init/")
    // 	require.NoError(t, err)
    //
    // 	db, err := databasesImplementation.NewSqlLiteInMemory()
    // 	require.NoError(t, err)
    //
    // 	tts := []struct {
    // 		name string
    // 	}{
    // 		{
    // 			name: "shifts",
    // 		},
    // 		{
    // 			name: "shifts_summary",
    // 		},
    // 	}
    // 	for _, tt := range tts {
    // 		t.Run(tt.name, func(t *testing.T) {
    // 			sql, _, err := lib.ProjectAndFsToQuerySql(p, f, tt.name)
    // 			require.NoError(t, err)
    // 			rows, err := db.QueryContext(ctx, sql)
    // 			require.NoError(t, err, fmt.Sprintf("sql: %s", sql))
    // 			countRows := 0
    // 			for rows.Next() {
    // 				countRows++
    // 			}
    // 			assert.Greater(t, countRows, 0)
    //
    // 			err = rows.Close()
    // 			require.NoError(t, err)
    // 		})
    // 	}
    // }
}
