use quine_mccluskey as qmc;

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables() {
    qmc::minimize_ex::<&str>(&[], &[], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables2() {
    qmc::minimize_minterms_ex::<&str>(&[], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn no_variables3() {
    qmc::minimize_maxterms_ex::<&str>(&[], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize_ex(
        &variables,
        &[],
        &[],
        qmc::SOP,
        qmc::MinimizeOptions::default(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables2() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize_minterms_ex(&variables, &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariableCount")]
fn too_many_variables3() {
    let mut variables = qmc::DEFAULT_VARIABLES.to_vec();
    variables.push("test");

    qmc::minimize_maxterms_ex(&variables, &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn variable_is_0() {
    qmc::minimize_ex(&["0"], &[], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn variable_is_0_2() {
    qmc::minimize_minterms_ex(&["0"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn variable_is_0_3() {
    qmc::minimize_maxterms_ex(&["0"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn variable_is_1() {
    qmc::minimize_ex(&["1"], &[], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn variable_is_1_2() {
    qmc::minimize_minterms_ex(&["1"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn variable_is_1_3() {
    qmc::minimize_maxterms_ex(&["1"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable() {
    qmc::minimize_ex(&[""], &[], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable2() {
    qmc::minimize_minterms_ex(&[""], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn empty_string_variable3() {
    qmc::minimize_maxterms_ex(&[""], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn leading_whitespace_variable() {
    qmc::minimize_ex(&[" A"], &[], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn leading_whitespace_variable2() {
    qmc::minimize_minterms_ex(&[" A"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn leading_whitespace_variable3() {
    qmc::minimize_maxterms_ex(&[" A"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn trailing_whitespace_variable() {
    qmc::minimize_ex(&["A "], &[], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn trailing_whitespace_variable2() {
    qmc::minimize_minterms_ex(&["A "], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidVariable")]
fn trailing_whitespace_variable3() {
    qmc::minimize_maxterms_ex(&["A "], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables() {
    qmc::minimize_ex(
        &["A", "B", "A"],
        &[],
        &[],
        qmc::SOP,
        qmc::MinimizeOptions::default(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables2() {
    qmc::minimize_minterms_ex(&["A", "B", "A"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "DuplicateVariables")]
fn duplicate_variables3() {
    qmc::minimize_maxterms_ex(&["A", "B", "A"], &[], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds() {
    qmc::minimize_ex(&["A"], &[2], &[], qmc::SOP, qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds2() {
    qmc::minimize_minterms_ex(&["A"], &[2], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "TermOutOfBounds")]
fn term_out_of_bounds3() {
    qmc::minimize_maxterms_ex(&["A"], &[2], &[], qmc::MinimizeOptions::default()).unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms() {
    qmc::minimize_ex(
        &["A", "B", "C"],
        &[0, 1, 2, 3],
        &[1, 4, 3],
        qmc::SOP,
        qmc::MinimizeOptions::default(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms2() {
    qmc::minimize_minterms_ex(
        &["A", "B", "C"],
        &[0, 1, 2, 3],
        &[1, 4, 3],
        qmc::MinimizeOptions::default(),
    )
    .unwrap();
}

#[test]
#[should_panic(expected = "TermConflict")]
fn conflicting_terms3() {
    qmc::minimize_maxterms_ex(
        &["A", "B", "C"],
        &[0, 1, 2, 3],
        &[1, 4, 3],
        qmc::MinimizeOptions::default(),
    )
    .unwrap();
}
