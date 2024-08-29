use crate::rpc_proto_scaffolding::{JsFileSystem, Writer};
use futures::AsyncReadExt;
use minijinja::Environment;
use quary_core::databases::DatabaseQueryGenerator;
use quary_core::file_system::FileSystem;
use quary_core::project::parse_project_files;
use quary_proto::{ReturnExplainModelPromptRequest, ReturnExplainModelPromptResponse};
use serde::Serialize;

pub async fn return_explain_model_prompt(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnExplainModelPromptRequest,
) -> Result<ReturnExplainModelPromptResponse, String> {
    let project_root = request.project_root.clone();
    let project =
        quary_core::project::parse_project(&file_system, &database, &project_root).await?;
    let model_name = request.model_name.clone();

    let model = project
        .models
        .get(&model_name)
        .ok_or_else(|| format!("Model {} not found in project", model_name))?;

    let mut sql = "".to_string();
    file_system
        .read_file(&model.file_path)
        .await
        .map_err(|e| format!("Error reading file {}: {}", model.file_path, e))?
        .read_to_string(&mut sql)
        .await
        .map_err(|e| format!("Error reading file {}: {}", model.file_path, e))?;

    let references = &model.references;
    let mut prompt_references: Vec<Reference> = vec![];
    for reference in references {
        let reference_model = project.models.get(reference);
        if let Some(reference_model) = reference_model {
            let mut reference_sql = "".to_string();
            file_system
                .read_file(&reference_model.file_path)
                .await
                .map_err(|e| format!("Error reading file {}: {}", reference_model.file_path, e))?
                .read_to_string(&mut reference_sql)
                .await
                .map_err(|e| format!("Error reading file {}: {}", reference_model.file_path, e))?;
            let reference_model_yaml = serde_yaml::to_string(reference_model).ok();
            prompt_references.push(Reference {
                model_name: reference.to_string(),
                sql: reference_sql,
                model_yaml: reference_model_yaml,
            });
        }
    }

    let parse_all_models = parse_project_files(&file_system, &project_root, &database).await?;
    let model_yaml = parse_all_models
        .iter()
        .find_map(|m| m.1.models.iter().find(|model| model.name == model_name));
    let model_yaml = model_yaml
        .map(|m| {
            serde_yaml::to_string(m)
                .map_err(|e| format!("Error serializing model yaml for {}: {}", model_name, e))
        })
        .transpose()?;
    let prompt = Prompt {
        database_type: database.get_name(),
        sql,
        model_name,
        model_yaml,
        references: prompt_references,
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

    Ok(ReturnExplainModelPromptResponse {
        agent_prompt: prompt,
        user_prompt: request.user_prompt,
    })
}

#[derive(Serialize)]
struct Prompt {
    database_type: &'static str,
    sql: String,
    model_name: String,
    model_yaml: Option<String>,
    references: Vec<Reference>,
}

#[derive(Serialize)]
struct Reference {
    model_name: String,
    sql: String,
    model_yaml: Option<String>,
}

const PROMPT_JINJA: &str = r#"
You are a data analyst at a company that uses a {{ database_type }} database. You've been asked to explain the following 
transformation called {{ model_name }} given to you in SQL:

```sql
{{ sql }}
```

{% if model_yaml %}
In addition to the sql transformation, here is the yaml document that contains the metadata for the model:

```yaml
{{ model_yaml }}
```
{% endif %}

The transformation is dependent on the following models where it references each of them in the SQL with the `q.` prefix:

{% for reference in references %}

Model: {{ reference.model_name }}

```sql
{{ reference.sql }}
```

{% if reference.model_yaml %}

In addition to the sql transformation, here is the yaml document that contains the metadata for {{ reference.model_name }}: 

```yaml
{{ reference.model_yaml }}
```

{% endif %}

{% endfor %}

Please explain the transformation in plain English answering the prompt given by the user.
"#;
