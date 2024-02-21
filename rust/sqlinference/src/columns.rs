use crate::dialect::Dialect;
use sqlparser::ast::Expr;

/// get_columns_internal returns the columns, as well as those not recognised in two vectors.
///
///
/// TODO: get_columns_internal could be also given a dependent map such that it can infer columns for underlying tables.
pub fn get_columns_internal(
    dialect: &Dialect,
    select_statement: &str,
) -> Result<(Vec<String>, Vec<String>), String> {
    let binding = dialect.get_dialect();
    let dialect = binding.as_ref();

    let ast = sqlparser::parser::Parser::parse_sql(dialect, select_statement);
    if let Err(err) = ast {
        return Err(err.to_string());
    };
    let Ok(ast) = ast else {
        return Err("No AST".to_string());
    };
    let ast = if let [ast] = &ast[..] {
        Ok(ast)
    } else {
        Err("Expected exactly one statement".to_string())
    }?;
    let ast = ast.clone();

    let mut columns: Vec<String> = vec![];
    let mut unnamed: Vec<String> = vec![];

    match ast {
        sqlparser::ast::Statement::Query(ref q) => {
            match *q.body {
                sqlparser::ast::SetExpr::Select(ref s) => {
                    for p in s.projection.clone() {
                        match p {
                            sqlparser::ast::SelectItem::UnnamedExpr(ref e) => {
                                match e {
                                    Expr::Identifier(ref i) => {
                                        columns.push(i.value.clone());
                                    }
                                    Expr::Value(ref v) => {
                                        unnamed.push(v.to_string());
                                    }
                                    Expr::Function(ref v) => {
                                        unnamed.push(v.to_string());
                                    }
                                    Expr::CompoundIdentifier(ref v) => {
                                        let value =
                                            v.last().ok_or("Expected Identifier or Value")?;
                                        columns.push(value.to_string());
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Expected Identifier or Value, not {:?}",
                                            e
                                        ));
                                    }
                                };
                            }
                            sqlparser::ast::SelectItem::ExprWithAlias { ref alias, .. } => {
                                columns.push(alias.value.clone());
                            }

                            _ => {
                                return Err(format!(
                                    "Expected UnnamedExpr or ExprWithAlias, not {:?}",
                                    p
                                ));
                            }
                        }
                    }
                }
                _ => return Err("Not a select".to_string()),
            };
        }
        _ => return Err("Not a query".to_string()),
    };
    Ok((columns, unnamed))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::dialect::Dialect;

    #[test]
    fn test_get_columns_internal() {
        let (cols, unnamed) = get_columns_internal(
            &Dialect::SQLite,
            "SELECT
            table_1.d,
            a,
            b AS c,
            123,
            my_func(b),
            SUM(strftime('%s', s.shift_end) - strftime('%s', s.shift_start)) AS total_hours
FROM table_1
WHERE a > b AND b < 100
ORDER BY a DESC, b",
        )
        .unwrap();
        assert_eq!(cols, vec!["d", "a", "c", "total_hours"]);
        assert_eq!(unnamed, vec!["123", "my_func(b)"]);
    }
}
