use crate::rpc_proto_scaffolding::{JsFileSystem, Writer};
use futures::AsyncReadExt;
use minijinja::Environment;
use quary_core::databases::DatabaseQueryGenerator;
use quary_core::file_system::FileSystem;
use quary_proto::{ReturnGenerateModelPromptRequest, ReturnGenerateModelPromptResponse};
use serde::Serialize;

pub async fn return_generate_model_prompt(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnGenerateModelPromptRequest,
) -> Result<ReturnGenerateModelPromptResponse, String> {
    let project_root = request.project_root.clone();
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;

    let mut references: Vec<Reference> = vec![];
    for model_name in request.references {
        let reference_model = project
            .models
            .get(&model_name)
            .ok_or(format!("Model {} not found in project", model_name))?;
        let mut reference_sql = "".to_string();
        file_system
            .read_file(&reference_model.file_path)
            .await
            .map_err(|e| format!("Error reading file {}: {}", reference_model.file_path, e))?
            .read_to_string(&mut reference_sql)
            .await
            .map_err(|e| format!("Error reading file {}: {}", reference_model.file_path, e))?;
        let reference_model_yaml = serde_yaml::to_string(reference_model).ok();
        references.push(Reference {
            model_name: model_name.to_string(),
            sql: reference_sql,
            model_yaml: reference_model_yaml,
        });
    }

    let prompt = Prompt {
        database_type: database.get_name(),
        references,
    };

    let mut env = Environment::new();
    env.add_template("1", PROMPT_JINJA)
        .map_err(|e| format!("Error adding template: {}", e))?; // .unwrap(
    let template = env
        .get_template("1")
        .map_err(|e| format!("Error getting template: {}", e))?;
    let prompt = template
        .render(prompt)
        .map_err(|e| format!("Error rendering template: {}", e))?;

    Ok(ReturnGenerateModelPromptResponse {
        user_prompt: request.prompt,
        agent_prompt: prompt,
    })
}

#[derive(Serialize)]
struct Prompt {
    database_type: &'static str,
    references: Vec<Reference>,
}

#[derive(Serialize)]
struct Reference {
    model_name: String,
    sql: String,
    model_yaml: Option<String>,
}

const PROMPT_JINJA: &str = r#"
Transform the following natural language requests into valid SQL queries. Assume a database with the following tables exist. Each table is 
from the SELECT statement and may contain some extra metadata provided in yaml file that describes the table and columns.

{% for reference in references %}

Table: {{ reference.model_name }}

CREATE TABLE {{ reference.model_name }} AS {{ reference.sql }}

Additional metadata for {{ reference.model_name }}:

{% if reference.model_yaml %}
{{ reference.model_yaml }}
{% endif %}

{% endfor %}

Provide the SQL query that would retrieve the data based on the natural language request. In addition to generating the SQL code,
please provide a brief explanation of the query. The SQL query should be valid for the database type {{ database_type }}.
The provided tables are available in the `q.` schema. To access tehm use the `q.` prefix, for example `q.table_name`.

A few general rules about the SQL queries you generate:
- Use the `q.` prefix to access the tables.
- The output must be a SELECT statement.
- The output must be a valid SQL query for the database type {{ database_type }}.
- Prefer CTEs over subqueries.
- In SELECT statements where you join two tables, when referring to columns, use the table alias.
- Prefer JOINs over subqueries.
- Use aliases for columns where the name is not clear or ambiguous, like on aggregate functions.
"#;
