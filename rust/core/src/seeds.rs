use crate::databases::DatabaseQueryGenerator;
use crate::file_system::convert_async_read_to_blocking_read;
use futures::AsyncRead;
use std::collections::HashSet;
use std::error::Error;

pub async fn parse_table_schema_seeds(
    database: &impl DatabaseQueryGenerator,
    table_name: &str,
    reader: Box<dyn AsyncRead + Send + Unpin>,
    do_not_include_data: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let rows = read_csv_to_strings(reader).await?;

    let lengths = rows.iter().map(|r| r.len()).collect::<HashSet<usize>>();
    if lengths.len() != 1 {
        return Err("expect all rows to have same length".into());
    }
    let drop_statement = database.seeds_drop_table_query(table_name);
    let columns = rows.first().ok_or("no columns present")?;
    let create_statement = database.seeds_create_table_query(table_name, columns);
    Ok(if rows.len() == 1 || do_not_include_data {
        vec![drop_statement, create_statement]
    } else {
        let rows = rows.get(1..).unwrap_or_default();
        let insert_statement = database.seeds_insert_into_table_query(table_name, columns, rows);
        vec![drop_statement, create_statement, insert_statement]
    })
}

async fn read_csv_to_strings(
    read: Box<dyn AsyncRead + Send + Unpin>,
) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let read = convert_async_read_to_blocking_read(read).await;

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(read);

    let mut rows: Vec<Vec<String>> = Vec::new();

    // TODO Make this functional
    for result in csv_reader.records() {
        let record = result.map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<String>>())?;
        rows.push(record);
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database_sqlite::DatabaseQueryGeneratorSqlite;

    #[tokio::test]
    async fn read_csv_to_strings_test() {
        let reader = Box::new(futures::io::Cursor::new("\"a',\",b,c\n1,2,3\n4,5,6"));

        let result = read_csv_to_strings(reader).await.unwrap();
        assert_eq!(
            result,
            vec![
                vec!["a',".to_string(), "b".to_string(), "c".to_string()],
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
                vec!["4".to_string(), "5".to_string(), "6".to_string()]
            ]
        );
    }

    #[tokio::test]
    async fn test_parse_table_schema_seeds() {
        let database = DatabaseQueryGeneratorSqlite {};

        let table_name = "test";
        let reader = Box::new(futures::io::Cursor::new("a,b,c\n1,2,3\n4,5,6"));
        let do_not_include_data = false;
        let result = parse_table_schema_seeds(&database, table_name, reader, do_not_include_data)
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![
                "DROP TABLE IF EXISTS test".to_string(),
                "CREATE TABLE test (a TEXT, b TEXT, c TEXT)".to_string(),
                "INSERT INTO test (a, b, c) VALUES ('1', '2', '3'), ('4', '5', '6')".to_string()
            ]
        );
    }

    #[tokio::test]
    async fn test_parse_table_schema_seeds_with_do_not_include_data() {
        let database = DatabaseQueryGeneratorSqlite {};

        let table_name = "test";
        let reader = Box::new(futures::io::Cursor::new("a,b,c\n1,2,3\n4,5,6"));
        let do_not_include_data = true;
        let result = parse_table_schema_seeds(&database, table_name, reader, do_not_include_data)
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![
                "DROP TABLE IF EXISTS test".to_string(),
                "CREATE TABLE test (a TEXT, b TEXT, c TEXT)".to_string(),
            ]
        );
    }

    #[tokio::test]
    async fn test_parse_table_schema_seeds_with_different_lengths() {
        let database = DatabaseQueryGeneratorSqlite {};

        let table_name = "test";
        let reader = Box::new(futures::io::Cursor::new("a,b,c\n1,2,3\n4,5"));
        let do_not_include_data = false;

        let result =
            parse_table_schema_seeds(&database, table_name, reader, do_not_include_data).await;

        assert!(result.is_err());
    }
}

// TODO Implement this test
// // TestDefaultSql_ParseTableSchemaSeeds_Reapply tests the reapplication of seeds to ensure they change the database
// // correctly consistently. The test applies various types of "reapplications".
// func TestDefaultSql_ParseTableSchemaSeeds_Reapply(t *testing.T) {
// 	t.Parallel()
//
// 	const original = `number,name
// 1,tom
// 2,jerry
// 3,david`
// 	const tableName = "test_table"
//
// 	tests := []struct {
// 		name         string
// 		replacingCSV string
// 		checkSQL     string
// 		wantValues   [][]string
// 	}{
// 		{
// 			name:         "same thing",
// 			replacingCSV: original,
// 			checkSQL:     "SELECT number AS TEXT, name FROM test_table ORDER BY number",
// 			wantValues: [][]string{
// 				{"1", "tom"},
// 				{"2", "jerry"},
// 				{"3", "david"},
// 			},
// 		},
// 		{
// 			name:         "added row",
// 			replacingCSV: original + "\n4,peter",
// 			checkSQL:     "SELECT number AS TEXT, name FROM test_table ORDER BY number",
// 			wantValues: [][]string{
// 				{"1", "tom"},
// 				{"2", "jerry"},
// 				{"3", "david"},
// 				{"4", "peter"},
// 			},
// 		},
// 		{
// 			name: "added column",
// 			replacingCSV: `number,name,last_name
// 1,tom,peters
// 2,jerry,smith
// 3,david,seagull`,
// 			checkSQL: "SELECT number AS TEXT, name, last_name FROM test_table ORDER BY number",
// 			wantValues: [][]string{
// 				{"1", "tom", "peters"},
// 				{"2", "jerry", "smith"},
// 				{"3", "david", "seagull"},
// 			},
// 		},
// 	}
//
// 	for _, tt := range tests {
// 		t.Run(tt.name, func(t *testing.T) {
// 			ctx := context.Background()
//
// 			// Database setup
// 			db, err := databasesImplementation.NewSqlLiteInMemory()
// 			require.NoError(t, err)
//
// 			// Apply original
// 			{
// 				sql, err := lib.ParseTableSchemaSeeds(db, tableName, strings.NewReader(original), false)
// 				require.NoError(t, err)
//
// 				_, err = db.ExecContext(ctx, strings.Join(sql, ";"))
// 				require.NoError(t, err)
// 			}
//
// 			// Check original value
// 			{
// 				rows, err := db.QueryContext(ctx, "SELECT number AS TEXT, name FROM test_table ORDER BY number")
// 				require.NoError(t, err)
// 				var values [][2]string
// 				for rows.Next() {
// 					var value [2]string
// 					err := rows.Scan(&value[0], &value[1])
// 					require.NoError(t, err)
// 					values = append(values, value)
// 				}
//
// 				require.Equal(t,
// 					[][2]string{
// 						{"1", "tom"},
// 						{"2", "jerry"},
// 						{"3", "david"},
// 					}, values)
// 			}
//
// 			// Apply new
// 			{
// 				sql, err := lib.ParseTableSchemaSeeds(db, tableName, strings.NewReader(tt.replacingCSV), false)
// 				require.NoError(t, err)
//
// 				_, err = db.ExecContext(ctx, strings.Join(sql, ";"))
// 				require.NoError(t, err)
// 			}
//
// 			// Check new values
// 			{
// 				rows, err := db.QueryContext(ctx, tt.checkSQL)
// 				require.NoError(t, err)
//
// 				var outs [][]string
// 				for rows.Next() {
// 					var out []string
//
// 					columns, err := rows.Columns()
// 					require.NoError(t, err)
//
// 					out = make([]string, len(columns))
// 					outPtrs := make([]interface{}, len(columns))
// 					for i := range columns {
// 						outPtrs[i] = &out[i]
// 					}
// 					err = rows.Scan(outPtrs...)
// 					require.NoError(t, err)
//
// 					outs = append(outs, out)
// 				}
//
// 				assert.Equal(t, tt.wantValues, outs)
// 			}
// 		})
// 	}
// }
