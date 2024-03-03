use quine_mccluskey as qmc;

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables() {
    qmc::minimize::<&str>(&[], &[], &[], true, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize(&variables, &[], &[], true, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable() {
    qmc::minimize(&[""], &[], &[], true, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn only_whitespace_variable() {
    qmc::minimize(&[" "], &[], &[], true, None).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables() {
    qmc::minimize(&["A", "B", "A", "C", "B"], &[], &[], true, None).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds() {
    qmc::minimize(&["A"], &[2], &[], true, None).unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms() {
    qmc::minimize(&["A", "B", "C"], &[0, 1, 2, 3], &[1, 4, 3], true, None).unwrap();
}
