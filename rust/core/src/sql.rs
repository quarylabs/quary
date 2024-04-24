use regex::{Error, Regex};

/// return_reference_search returns a Regex searcher that can be used to search for references to
/// models inside quary. It will look for matches that match the schema name that can be used for
/// further processing.
pub fn return_reference_search(schema_name: &str) -> Result<Regex, Error> {
    Regex::new(format!(r#"\s{}\.([a-zA-Z][a-z_A-Z0-9]*)"#, schema_name).as_str())
}

/// remove_sql_comments removes comments from an SQL string. It will remove both single line comments
/// and multi-line comments.
pub fn remove_sql_comments(sql: &str) -> String {
    #[allow(clippy::unwrap_used)]
    let single_line_comment = Regex::new(r"--.*").unwrap();
    #[allow(clippy::unwrap_used)]
    let multi_line_comment = Regex::new(r"/\*.*\*/").unwrap();

    let result = single_line_comment.replace_all(sql, "");
    let output = multi_line_comment.replace_all(result.as_ref(), "");

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_sql_comments() {
        let sql = "SELECT * FROM test;
-- This is a comment
SELECT * FROM test2;
/* This is a multi-line comment */";
        let expected = "SELECT * FROM test;\n\nSELECT * FROM test2;\n";

        assert_eq!(expected, remove_sql_comments(sql));
    }

    #[test]
    fn test_return_reference_search() {
        let schema_name = "public";
        let regex = return_reference_search(schema_name).unwrap();

        assert_eq!(r"\spublic\.([a-zA-Z][a-z_A-Z0-9]*)", regex.as_str());

        let matching_string = "SELECT a FROM public.test";
        let non_matching_string = "SELECT a FROM not_public.test";

        assert!(regex.is_match(matching_string));
        assert!(!regex.is_match(non_matching_string));
    }
}
