use quary_proto::{Position, Range};
use std::collections::HashMap;

/// Sql_model_finder is a function that takes an sql string and returns a list of models it finds in
/// the sql string.
///
/// It finds the models by looking for `q.*` with in the sql string and then extracting the model name. It
/// returns a list of the unique model names with the `q.` prefix removed as well as the positions
/// of those references
pub fn sql_model_finder(sql: &str) -> HashMap<String, Vec<Range>> {
    let mut result = HashMap::new();
    let mut line = 0;
    let mut character = 0;
    for (i, c) in sql.chars().enumerate() {
        if c == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }

        if c == 'q' {
            if let Some('.') = sql.chars().nth(i + 1) {
                let mut j = i + 2;
                let mut model = String::new();
                while j < sql.len() {
                    if let Some(c) = sql.chars().nth(j) {
                        if c == ' ' || c == '\n' || c == '\t' || c == ',' || c == ')' {
                            break;
                        }
                        model.push(c);
                        j += 1;
                    }
                }
                result
                    .entry(model.clone())
                    .or_insert_with(Vec::new)
                    .push(Range {
                        start: Some(Position { line, character }),
                        end: Some(Position {
                            line,
                            character: (character + model.len() as u32),
                        }),
                    });
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_model_finder_single_line() {
        let sql = "SELECT hello FROM q.foo";

        let result = sql_model_finder(sql);

        let expected = HashMap::from([(
            "foo".to_string(),
            vec![Range {
                start: Some(Position {
                    line: 0,
                    character: 19,
                }),
                end: Some(Position {
                    line: 0,
                    character: 22,
                }),
            }],
        )]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_sql_model_finder_none() {
        let sql = "SELECT hello FROM temp";

        let result = sql_model_finder(sql);

        let expected = HashMap::from([]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_sql_model_finder_multiline() {
        let sql = "SELECT hello FROM q.foo AS foo\nINNER JOIN q.bar AS bar ON bar.id = foo.id";
        let result = sql_model_finder(sql);

        let expected = HashMap::from([
            (
                "foo".to_string(),
                vec![Range {
                    start: Some(Position {
                        line: 0,
                        character: 19,
                    }),
                    end: Some(Position {
                        line: 0,
                        character: 22,
                    }),
                }],
            ),
            (
                "bar".to_string(),
                vec![Range {
                    start: Some(Position {
                        line: 1,
                        character: 12,
                    }),
                    end: Some(Position {
                        line: 1,
                        character: 15,
                    }),
                }],
            ),
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_sql_model_finder_reference_same_model_twice() {
        let sql = "SELECT hello FROM q.foo AS foo\nINNER JOIN q.foo AS bar ON bar.id = foo.id";
        let result = sql_model_finder(sql);

        let expected = HashMap::from([(
            "foo".to_string(),
            vec![
                Range {
                    start: Some(Position {
                        line: 0,
                        character: 19,
                    }),
                    end: Some(Position {
                        line: 0,
                        character: 22,
                    }),
                },
                Range {
                    start: Some(Position {
                        line: 1,
                        character: 12,
                    }),
                    end: Some(Position {
                        line: 1,
                        character: 15,
                    }),
                },
            ],
        )]);

        assert_eq!(expected, result);
    }
}
