use crate::dialect::Dialect;
use crate::infer_tests::Operation;
use crate::test::Test;
use std::collections::HashMap;

pub enum TreatmentOfNullExpressionsInAggregates {
    /// IgnoreNullExpressions is Postgres like in that aggregate functions ignore non-null values.
    IgnoreNullExpressions,
    // TODO Implement this
    // NullExpressionsDominate,
}

pub fn aggregate_function_behaviour(
    dialect: &Dialect,
) -> HashMap<Operation, TreatmentOfNullExpressionsInAggregates> {
    match dialect {
        Dialect::SQLite | Dialect::BigQuery | Dialect::Snowflake | Dialect::DuckDB => {
            HashMap::from([
                (
                    Operation::Min,
                    TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
                ),
                (
                    Operation::Max,
                    TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
                ),
                (
                    Operation::Avg,
                    TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
                ),
            ])
        }
    }
}

pub fn inferred_through_aggregate_function(
    treatment_of_null_expressions_in_aggregates: &TreatmentOfNullExpressionsInAggregates,
    group_by: &bool,
    test: &Test,
) -> bool {
    match (treatment_of_null_expressions_in_aggregates, group_by, test) {
        (
            TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
            false,
            Test::NotNull(_),
        ) => false,
        (TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions, true, Test::NotNull(_)) => {
            true
        }
        (TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions, _, Test::Unique(_)) => true,
        (
            TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
            _,
            Test::Relationship(_),
        ) => true,
        (
            TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
            _,
            Test::AcceptedValues(_),
        ) => true,
        (
            TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
            _,
            Test::GreaterThanOrEqual(_),
        ) => true,
        (
            TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
            _,
            Test::GreaterThan(_),
        ) => true,
        (
            TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions,
            _,
            Test::LessThanOrEqual(_),
        ) => true,
        (TreatmentOfNullExpressionsInAggregates::IgnoreNullExpressions, _, Test::LessThan(_)) => {
            true
        }
    }
}

pub fn aggregate_is_test_inferrable(
    dialect: &Dialect,
    test: &Test,
    operation: &Operation,
    group_by: &bool,
) -> bool {
    let aggregate_function_behaviour = aggregate_function_behaviour(dialect);
    let treatment = aggregate_function_behaviour.get(operation);
    if let Some(treatment) = treatment {
        inferred_through_aggregate_function(treatment, group_by, test)
    } else {
        false
    }
}
