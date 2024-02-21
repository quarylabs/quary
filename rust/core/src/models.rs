use crate::databases::DatabaseQueryGenerator;
use crate::sql::return_reference_search;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::Read;

#[allow(clippy::unwrap_used)]
static VALIDATE_CONFIG_SCHEMA_NAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z][a-z0-9_]*$").unwrap());

// Function to validate the model name
pub fn validate_model_name(name: &str) -> Result<(), String> {
    // Check if the model name matches the regex pattern
    if !VALIDATE_CONFIG_SCHEMA_NAME.is_match(name) {
        return Err(format!(
            "model name must match {}",
            *VALIDATE_CONFIG_SCHEMA_NAME
        ));
    }

    Ok(())
}

/// parse_model_schemas_to_views takes in a reader and reads it to a View file
/// name_replacing_strategy takes in the reference name and replaces it with whatever strategy is necessary.
pub fn parse_model_schemas_to_views<F>(
    database: &impl DatabaseQueryGenerator,
    file_reader: Box<dyn Read>,
    view_name: &str,
    config_schema_name: &str,
    name_replacing_strategy: F,
) -> Result<[String; 2], String>
where
    F: Fn(&regex::Captures) -> String,
{
    let original_select_statement = read_normalise_model(file_reader)?;

    let reference_search =
        return_reference_search(config_schema_name).map_err(|e| e.to_string())?;

    let out_select =
        reference_search.replace_all(&original_select_statement, name_replacing_strategy);

    Ok(return_sql_model_template(database, view_name, &out_select))
}

pub fn read_normalise_model(mut file_reader: Box<dyn Read>) -> Result<String, String> {
    let mut buf = String::new();
    file_reader
        .read_to_string(&mut buf)
        .map_err(|e| e.to_string())?;

    Ok(buf.trim().strip_suffix(';').unwrap_or(&buf).to_string())
}

fn return_sql_model_template(
    database: &impl DatabaseQueryGenerator,
    name: &str,
    select_statement: &str,
) -> [String; 2] {
    let drop = database.models_drop_view_query(name);
    let create = database.models_create_view_query(name, select_statement);
    [drop, create]
}

#[cfg(test)]
mod test {
    //     TODO Implement tests
    //     func TestParseModelSchemasToViews(t *testing.T) {
    // 	t.Parallel()
    //
    // 	type args struct {
    // 		fileReader            io.Reader
    // 		tableName             string
    // 		nameReplacingStrategy func(name string) string
    // 	}
    // 	tests := []struct {
    // 		name    string
    // 		args    args
    // 		want    string
    // 		wantErr assert.ErrorAssertionFunc
    // 	}{
    // 		{
    // 			name: "simple example",
    // 			args: args{
    // 				fileReader:            bytes.NewBufferString("SELECT * FROM q.users"),
    // 				tableName:             "view_name",
    // 				nameReplacingStrategy: lib.ReplaceReferenceStringFound("q", map[string]*servicev1.Source{}),
    // 			},
    // 			want:    "DROP VIEW IF EXISTS view_name; CREATE VIEW view_name AS SELECT * FROM users;",
    // 			wantErr: assert.NoError,
    // 		},
    // 		{
    // 			name: "simple example that also has a source",
    // 			args: args{
    // 				fileReader: bytes.NewBufferString("SELECT * FROM q.users"),
    // 				tableName:  "view_name",
    // 				nameReplacingStrategy: lib.ReplaceReferenceStringFound("q", map[string]*servicev1.Source{
    // 					"users": {
    // 						Name:     "users",
    // 						Path:     "schema.users_123",
    // 						FilePath: "models/test.yaml",
    // 						Columns:  nil,
    // 					},
    // 				}),
    // 			},
    // 			want:    "DROP VIEW IF EXISTS view_name; CREATE VIEW view_name AS SELECT * FROM schema.users_123;",
    // 			wantErr: assert.NoError,
    // 		},
    // 	}
    // 	for _, tt := range tests {
    // 		t.Run(tt.name, func(t *testing.T) {
    // 			got, err := lib.ParseModelSchemasToViews(tt.args.fileReader, tt.args.tableName, "q", tt.args.nameReplacingStrategy)
    // 			if !tt.wantErr(t, err, fmt.Sprintf("ParseModelSchemasToViews(%v, %v)", tt.args.fileReader, tt.args.tableName)) {
    // 				return
    // 			}
    // 			assert.Equalf(t, tt.want, got, "ParseModelSchemasToViews(%v, %v, %v)", tt.args.fileReader, tt.args.tableName, tt.args.nameReplacingStrategy)
    // 		})
    // 	}
    // }
    //
    // // TestParseModelSchemasToViews_ReapplyingModel tests that the model can be reapplied to the database
    // // without error and the updated data be returned correctly to an in-memory SQLLite database.
    // func TestParseModelSchemasToViews_ReapplyingModelSqlLite(t *testing.T) {
    // 	t.Parallel()
    //
    // 	ctx := context.Background()
    //
    // 	// Setting up database
    // 	db, err := databasesImplementation.NewSqlLiteInMemory()
    // 	require.NoError(t, err)
    //
    // 	// Setting up base table to query from view
    // 	_, err = db.ExecContext(ctx, "CREATE TABLE users (id INTEGER PRIMARY KEY, user_name TEXT);")
    //
    // 	// Applying the view first time around
    // 	const viewName = "reused_view_name"
    // 	const firstQuery = "SELECT id AS id_int, user_name as name FROM users"
    //
    // 	model, err := lib.ParseModelSchemasToViews(
    // 		bytes.NewBufferString(firstQuery),
    // 		viewName,
    // 		"q",
    // 		func(name string) string { return name },
    // 	)
    // 	require.NoError(t, err)
    // 	_, err = db.ExecContext(ctx, model)
    // 	require.NoError(t, err)
    // 	_, err = db.ExecContext(ctx, "SELECT id_int, name FROM reused_view_name")
    // 	require.NoError(t, err)
    //
    // 	// Applying the same view and checking it
    // 	model, err = lib.ParseModelSchemasToViews(
    // 		bytes.NewBufferString(firstQuery),
    // 		viewName,
    // 		"q",
    // 		func(name string) string { return name },
    // 	)
    // 	require.NoError(t, err)
    // 	_, err = db.ExecContext(ctx, model)
    // 	require.NoError(t, err)
    // 	_, err = db.ExecContext(ctx, "SELECT id_int, name FROM reused_view_name")
    // 	require.NoError(t, err)
    //
    // 	// Slightly checking view and checking it
    // 	model, err = lib.ParseModelSchemasToViews(
    // 		bytes.NewBufferString("SELECT id AS id_int_again, user_name as name FROM users"),
    // 		viewName,
    // 		"q",
    // 		func(name string) string { return name },
    // 	)
    // 	require.NoError(t, err)
    // 	_, err = db.ExecContext(ctx, model)
    // 	require.NoError(t, err)
    // 	_, err = db.ExecContext(ctx, "SELECT id_int, name FROM reused_view_name")
    // 	require.Error(t, err)
    // 	_, err = db.ExecContext(ctx, "SELECT id_int_again, name FROM reused_view_name")
    // 	require.NoError(t, err)
    // }
    //
    // func BenchmarkParseModelSchemasToViews(b *testing.B) {
    // 	for i := 0; i < b.N; i++ {
    // 		_, err := lib.ParseModelSchemasToViews(
    // 			bytes.NewBufferString("SELECT * FROM q.users"),
    // 			"view_name",
    // 			"q",
    // 			func(name string) string { return name },
    // 		)
    // 		if err != nil {
    // 			b.Fatal(err)
    // 		}
    // 	}
    // }
}
