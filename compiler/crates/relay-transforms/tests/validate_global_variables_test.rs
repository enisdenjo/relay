// @generated SignedSource<<705e68c007af14e73525dfba0a396f65>>
// Generated by $ cargo run -p fixture-tests -- oss/crates/relay-transforms/tests/validate_global_variables

mod validate_global_variables;

use validate_global_variables::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn fragment_with_undefined_variable_invalid() {
    let input = include_str!("validate_global_variables/fixtures/fragment-with-undefined-variable.invalid.graphql");
    let expected = include_str!("validate_global_variables/fixtures/fragment-with-undefined-variable.invalid.expected");
    test_fixture(transform_fixture, "fragment-with-undefined-variable.invalid.graphql", "validate_global_variables/fixtures/fragment-with-undefined-variable.invalid.expected", input, expected);
}

#[test]
fn query_with_undefined_variable_invalid() {
    let input = include_str!("validate_global_variables/fixtures/query-with-undefined-variable.invalid.graphql");
    let expected = include_str!("validate_global_variables/fixtures/query-with-undefined-variable.invalid.expected");
    test_fixture(transform_fixture, "query-with-undefined-variable.invalid.graphql", "validate_global_variables/fixtures/query-with-undefined-variable.invalid.expected", input, expected);
}

#[test]
fn query_with_variables() {
    let input = include_str!("validate_global_variables/fixtures/query-with-variables.graphql");
    let expected = include_str!("validate_global_variables/fixtures/query-with-variables.expected");
    test_fixture(transform_fixture, "query-with-variables.graphql", "validate_global_variables/fixtures/query-with-variables.expected", input, expected);
}
