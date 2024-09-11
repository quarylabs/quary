use crate::rpc_proto_scaffolding::{JsFileSystem, Writer};
use minijinja::Environment;
use quary_core::databases::DatabaseQueryGenerator;
use quary_core::file_system::FileSystem;
use quary_proto::{
    ReturnGenerateModelPromptToSearchForModelsRequest,
    ReturnGenerateModelPromptToSearchForModelsResponse,
};
use serde::Serialize;

pub async fn return_generate_model_prompt_to_search_for_models(
    database: impl DatabaseQueryGenerator,
    _: Writer,
    file_system: JsFileSystem,
    request: ReturnGenerateModelPromptToSearchForModelsRequest,
) -> Result<ReturnGenerateModelPromptToSearchForModelsResponse, String> {
    return_generate_model_prompt_to_search_for_models_internal(&database, &file_system, request)
        .await
}

async fn return_generate_model_prompt_to_search_for_models_internal(
    database: &impl DatabaseQueryGenerator,
    file_system: &impl FileSystem,
    request: ReturnGenerateModelPromptToSearchForModelsRequest,
) -> Result<ReturnGenerateModelPromptToSearchForModelsResponse, String> {
    let project_root = request.project_root.clone();
    let project = quary_core::project::parse_project(file_system, database, &project_root).await?;

    let mut models: Vec<Model> = project
        .models
        .into_iter()
        .map(|(model, details)| Model {
            name: model,
            description: details
                .description
                .unwrap_or("No description available".to_string()),
        })
        .collect();
    // sort the models by the model name
    models.sort_by(|a, b| a.name.cmp(&b.name));

    let prompt = Prompt { models };
    let mut env = Environment::new();
    env.add_template("1", PROMPT_JINJA)
        .map_err(|e| format!("Error adding template: {}", e))?; // .unwrap(
    let template = env
        .get_template("1")
        .map_err(|e| format!("Error getting template: {}", e))?;
    let prompt = template
        .render(prompt)
        .map_err(|e| format!("Error rendering template: {}", e))?;

    Ok(ReturnGenerateModelPromptToSearchForModelsResponse {
        user_prompt: request.prompt,
        agent_prompt: prompt,
    })
}

#[derive(Serialize)]
struct Prompt {
    models: Vec<Model>,
}

#[derive(Serialize)]
struct Model {
    name: String,
    description: String,
}

const PROMPT_JINJA: &str = r#"
You are an agent that is helps users generate queries for their database to answer prompts. Return 
a list of models of the following tables of model names and optional descriptions that are likely to 
be useful in answering the prompt.

{% for model in models %}
{{ model.name }}: {{ model.description }}
{% endfor %}

Only return a JSON array with the names of the models that are likely to be useful in answering the prompt.
"#;

#[cfg(test)]
mod tests {
    use crate::rpc_proto_prompt_functions_generate_to_search::return_generate_model_prompt_to_search_for_models_internal;
    use quary_core::database_sqlite::DatabaseQueryGeneratorSqlite;
    use quary_core::init::Asset;

    #[tokio::test]
    async fn test_return_generate_model_prompt_to_search_for_models() {
        let assets = Asset {};

        let user_prompt = "Users prompt".to_string();
        let request = quary_proto::ReturnGenerateModelPromptToSearchForModelsRequest {
            project_root: "".to_string(),
            prompt: user_prompt.clone(),
        };
        let database = DatabaseQueryGeneratorSqlite::default();

        let response =
            return_generate_model_prompt_to_search_for_models_internal(&database, &assets, request)
                .await
                .unwrap();

        assert_eq!(response.user_prompt, user_prompt);
        assert!(response
            .agent_prompt
            .contains("Only return a JSON array with the names of the models"));
        assert!(response.agent_prompt.contains("shifts_summary"));
    }
}
