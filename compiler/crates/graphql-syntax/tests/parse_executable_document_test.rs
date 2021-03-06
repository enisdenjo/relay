// @generated SignedSource<<a2458949db830ebc4dcc825a0bef5dbb>>
// Generated by $ cargo run -p fixture-tests -- oss/crates/graphql-syntax/tests/parse_executable_document

mod parse_executable_document;

use parse_executable_document::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn incorrect_variable_name_invalid() {
    let input = include_str!("parse_executable_document/fixtures/incorrect_variable_name.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/incorrect_variable_name.invalid.expected");
    test_fixture(transform_fixture, "incorrect_variable_name.invalid.graphql", "parse_executable_document/fixtures/incorrect_variable_name.invalid.expected", input, expected);
}

#[test]
fn invalid_number() {
    let input = include_str!("parse_executable_document/fixtures/invalid_number.graphql");
    let expected = include_str!("parse_executable_document/fixtures/invalid_number.expected");
    test_fixture(transform_fixture, "invalid_number.graphql", "parse_executable_document/fixtures/invalid_number.expected", input, expected);
}

#[test]
fn keyword_as_name() {
    let input = include_str!("parse_executable_document/fixtures/keyword_as_name.graphql");
    let expected = include_str!("parse_executable_document/fixtures/keyword_as_name.expected");
    test_fixture(transform_fixture, "keyword_as_name.graphql", "parse_executable_document/fixtures/keyword_as_name.expected", input, expected);
}

#[test]
fn kitchen_sink() {
    let input = include_str!("parse_executable_document/fixtures/kitchen-sink.graphql");
    let expected = include_str!("parse_executable_document/fixtures/kitchen-sink.expected");
    test_fixture(transform_fixture, "kitchen-sink.graphql", "parse_executable_document/fixtures/kitchen-sink.expected", input, expected);
}

#[test]
fn list_of_enum() {
    let input = include_str!("parse_executable_document/fixtures/list_of_enum.graphql");
    let expected = include_str!("parse_executable_document/fixtures/list_of_enum.expected");
    test_fixture(transform_fixture, "list_of_enum.graphql", "parse_executable_document/fixtures/list_of_enum.expected", input, expected);
}

#[test]
fn missing_zero_on_float_invalid() {
    let input = include_str!("parse_executable_document/fixtures/missing_zero_on_float.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/missing_zero_on_float.invalid.expected");
    test_fixture(transform_fixture, "missing_zero_on_float.invalid.graphql", "parse_executable_document/fixtures/missing_zero_on_float.invalid.expected", input, expected);
}

#[test]
fn multiple_parse_errors_invalid() {
    let input = include_str!("parse_executable_document/fixtures/multiple_parse_errors.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/multiple_parse_errors.invalid.expected");
    test_fixture(transform_fixture, "multiple_parse_errors.invalid.graphql", "parse_executable_document/fixtures/multiple_parse_errors.invalid.expected", input, expected);
}

#[test]
fn space_in_variable() {
    let input = include_str!("parse_executable_document/fixtures/space_in_variable.graphql");
    let expected = include_str!("parse_executable_document/fixtures/space_in_variable.expected");
    test_fixture(transform_fixture, "space_in_variable.graphql", "parse_executable_document/fixtures/space_in_variable.expected", input, expected);
}

#[test]
fn unterminated_string_invalid() {
    let input = include_str!("parse_executable_document/fixtures/unterminated_string.invalid.graphql");
    let expected = include_str!("parse_executable_document/fixtures/unterminated_string.invalid.expected");
    test_fixture(transform_fixture, "unterminated_string.invalid.graphql", "parse_executable_document/fixtures/unterminated_string.invalid.expected", input, expected);
}
