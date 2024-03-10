use quine_mccluskey as qmc;

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables() {
    qmc::minimize::<&str>(&[], &[], &[], true, false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables2() {
    qmc::minimize_minterms::<&str>(&[], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables3() {
    qmc::minimize_maxterms::<&str>(&[], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize(&variables, &[], &[], true, false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables2() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize_minterms(&variables, &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables3() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize_maxterms(&variables, &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable() {
    qmc::minimize(&[""], &[], &[], true, false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable2() {
    qmc::minimize_minterms(&[""], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable3() {
    qmc::minimize_maxterms(&[""], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn only_whitespace_variable() {
    qmc::minimize(&[" "], &[], &[], true, false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn only_whitespace_variable2() {
    qmc::minimize_minterms(&[" "], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn only_whitespace_variable3() {
    qmc::minimize_maxterms(&[" "], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables() {
    qmc::minimize(&["A", "B", "A"], &[], &[], true, false, None).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables2() {
    qmc::minimize_minterms(&["A", "B", "A"], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables3() {
    qmc::minimize_maxterms(&["A", "B", "A"], &[], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds() {
    qmc::minimize(&["A"], &[2], &[], true, false, None).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds2() {
    qmc::minimize_minterms(&["A"], &[2], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds3() {
    qmc::minimize_maxterms(&["A"], &[2], &[], false, None).unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms() {
    qmc::minimize(
        &["A", "B", "C"],
        &[0, 1, 2, 3],
        &[1, 4, 3],
        true,
        false,
        None,
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms2() {
    qmc::minimize_minterms(&["A", "B", "C"], &[0, 1, 2, 3], &[1, 4, 3], false, None).unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms3() {
    qmc::minimize_maxterms(&["A", "B", "C"], &[0, 1, 2, 3], &[1, 4, 3], false, None).unwrap();
}
