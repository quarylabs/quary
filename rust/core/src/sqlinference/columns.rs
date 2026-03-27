use sqruff_lib_core::dialects::syntax::{SyntaxKind, SyntaxSet};
use sqruff_lib_core::parser::Parser;
use sqruff_lib_core::utils::analysis::query::Query;

use super::parse_sql;

/// get_columns_internal returns the columns, as well as those not recognised in
/// two vectors.
///
///
/// TODO: get_columns_internal could be also given a dependent map such that it
/// can infer columns for underlying tables.
pub fn get_columns_internal(
    parser: &Parser,
    select_statement: &str,
) -> Result<(Vec<String>, Vec<String>), String> {
    let ast = parse_sql(parser, select_statement);

    let mut columns: Vec<String> = vec![];
    let mut unnamed: Vec<String> = vec![];

    let query: Query<'_> = Query::from_root(&ast, parser.dialect()).unwrap();
    let ast = query.inner.borrow().selectables[0].selectable.clone();

    for segment in ast.recursive_crawl(
        const { &SyntaxSet::new(&[SyntaxKind::SelectClauseElement]) },
        false,
        const { &SyntaxSet::single(SyntaxKind::SelectStatement) },
        false,
    ) {
        if let Some(alias) =
            segment.child(const { &SyntaxSet::new(&[SyntaxKind::AliasExpression]) })
        {
            let raw_segments = alias.get_raw_segments();
            let alias = raw_segments.iter().rev().find(|it| it.is_code()).unwrap();
            columns.push(alias.raw().to_string());
            continue;
        }

        for segment in segment.segments() {
            match segment.get_type() {
                SyntaxKind::ColumnReference => {
                    let value = segment.get_raw_segments().last().unwrap().clone();
                    columns.push(value.raw().to_string());
                }
                _ => {
                    unnamed.push(segment.raw().to_string());
                }
            }
        }
    }

    Ok((columns, unnamed))
}

#[cfg(test)]
mod tests {
    use sqruff_lib_core::parser::Parser;
    use sqruff_lib_dialects::ansi;

    use super::*;

    #[test]
    fn test_get_columns_internal() {
        let dialect = ansi::dialect();
        let parser = Parser::from(&dialect);

        let (cols, unnamed) = get_columns_internal(
            &parser,
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

    #[test]
    fn test_sub_query() {
        let dialect = ansi::dialect();
        let parser = Parser::from(&dialect);

        let (cols, unnamed) = get_columns_internal(
            &parser,
            "
SELECT
    account_id,
    valid_date,
    gross_as_percentage
FROM
    (
        SELECT
            account_id,
            valid_date,
            gross_as_percentage
        FROM
            q.stg_savings_account_rates
    )",
        )
        .unwrap();
        assert_eq!(
            cols,
            vec!["account_id", "valid_date", "gross_as_percentage"]
        );
        assert_eq!(unnamed, Vec::<String>::new());
    }
}
