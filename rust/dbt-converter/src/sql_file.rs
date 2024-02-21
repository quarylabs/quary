use regex::{Captures, Regex};
use std::io;

pub(crate) fn translate_sql_file(mut reader: impl io::Read) -> Result<String, String> {
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .map_err(|e| e.to_string())?;

    // translate Jinja comments to SQL comments
    content = translate_jinja_comment_to_sql_comment(&content);
    content = translate_dbt_config_to_sql_comment(&content);

    // regex for refs (model references)
    let re_ref = Regex::new(r#"\{\{\s*ref\s*\(['"]([a-zA-Z0-9_]+)['"]\)\s*\}\}"#)
        .map_err(|e| e.to_string())?;
    content = re_ref.replace_all(&content, "q.$1").to_string();

    // regex for source (source references)
    let re_source = Regex::new(r"\{\{\s*source\('([a-zA-Z0-9_]+)',\s*'([a-zA-Z0-9_]+)'\)\s*\}\}")
        .map_err(|e| e.to_string())?;
    content = re_source
        .replace_all(&content, |caps: &Captures| {
            format!("q.{}_{}", &caps[1], &caps[2])
        })
        .to_string();

    Ok(content)
}

/// translate_jinja_comment_to_sql_comment translates jinja comments to sql comments going from a {# #} to /* */ syntax.
fn translate_jinja_comment_to_sql_comment(content: &str) -> String {
    let re = Regex::new(r"(?s)\{\#(.+?)\#\}").unwrap();
    re.replace_all(content, |caps: &Captures| format!("/*{}*/", &caps[1]))
        .to_string()
}

/// translate_dbt_config_to_sql_comment converts unsupported DBT config blocks to SQL comments.
fn translate_dbt_config_to_sql_comment(content: &str) -> String {
    let re_config = Regex::new(r"\{\{\s*config\(([\s\S]*?)\)\s*\}\}").unwrap();
    re_config
        .replace_all(content, |caps: &Captures| {
            // This time, ensure the conversion does not add * on new lines within the comment
            format!("/* config({}) */", &caps[1])
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn simple_ref_translation_sql_file() {
        let sql_file = r#"
            SELECT
                *
            FROM
                {{ ref('underlying_model') }}
        "#;
        let expected = r#"
            SELECT
                *
            FROM
                q.underlying_model
        "#;

        let reader = Cursor::new(sql_file);

        assert_eq!(Ok(expected.to_string()), translate_sql_file(reader));
    }

    #[test]
    fn simple_ref_with_space_translation_sql_file() {
        let sql_file = r#"
        select *
        from {{ ref ('stg_backend__transactions_collection') }}
        where state = 'COMPLETED'
        "#;
        let expected = r#"
        select *
        from q.stg_backend__transactions_collection
        where state = 'COMPLETED'
        "#;

        let reader = Cursor::new(sql_file);

        assert_eq!(Ok(expected.to_string()), translate_sql_file(reader));
    }

    #[test]
    fn simple_ref_with_double_quotes_translation_sql_file() {
        let sql_file = r#"
            with base_estimated_balance as (
                select *
                from {{ ref("int_cover_rate_weighted_average") }}
            )
        "#;
        let expected = r#"
            with base_estimated_balance as (
                select *
                from q.int_cover_rate_weighted_average
            )
        "#;

        let reader = Cursor::new(sql_file);

        assert_eq!(Ok(expected.to_string()), translate_sql_file(reader));
    }

    #[test]
    fn simple_source_translation_sql_file() {
        let sql_file = r#"
            SELECT
                *
            FROM
                {{ source('source_name','source_table') }}
        "#;
        let expected = r#"
            SELECT
                *
            FROM
                q.source_name_source_table
        "#;

        let reader = Cursor::new(sql_file);

        assert_eq!(Ok(expected.to_string()), translate_sql_file(reader));
    }

    #[test]
    fn test_translate_jinja_comment_to_sql_comment() {
        let sql_file = r#"
            SELECT
                *
            FROM
                {{ ref('underlying_model') }}
            {# This is a comment #}
        "#;
        let expected = "\n            SELECT\n                *\n            FROM\n                {{ ref('underlying_model') }}\n            /* This is a comment */\n        ";

        assert_eq!(
            expected.to_string(),
            translate_jinja_comment_to_sql_comment(sql_file)
        );
    }

    #[test]
    fn test_translate_jinja_comment_to_sql_comment_multi_line() {
        let sql_file = r#"
            SELECT
                *
            FROM
                {{ ref('underlying_model') }}
            {# 
               This is a multi-line comment
             #}    
        "#;
        let expected = "\n            SELECT\n                *\n            FROM\n                {{ ref('underlying_model') }}\n            /* \n               This is a multi-line comment\n             */    \n        ";

        assert_eq!(
            expected.to_string(),
            translate_jinja_comment_to_sql_comment(sql_file)
        );
    }

    #[test]
    fn test_translate_dbt_config_to_sql_comment() {
        let sql_file_with_config = r#"
            {{
                config(
                    materialized='view'
                )
            }}
            SELECT
                *
            FROM
                {{ ref('underlying_model') }}
        "#;
        let expected_output =   "\n            /* config(\n                    materialized='view'\n                ) */\n            SELECT\n                *\n            FROM\n                {{ ref('underlying_model') }}\n        ";

        assert_eq!(
            expected_output.to_string(),
            translate_dbt_config_to_sql_comment(sql_file_with_config)
        );
    }
}
