use std::collections::HashSet;

use quine_mccluskey as qmc;

#[test]
fn solution() {
    test(1, &[], &[0, 1], "0", "0", "0", "0");
    test(1, &[0], &[1], "~A", "~A", "~A", "~A");
    test(1, &[1], &[0], "A", "A", "A", "A");
    test(1, &[0, 1], &[], "1", "1", "1", "1");
    test(1, &[], &[], "0", "0", "1", "1");
    test(1, &[], &[0], "0", "0", "0", "0");
    test(1, &[], &[1], "0", "0", "0", "0");
    test(1, &[0], &[], "1", "1", "1", "1");
    test(1, &[1], &[], "1", "1", "1", "1");

    test(2, &[], &[], "0", "0", "1", "1");
    test(2, &[1], &[0, 3], "~A ∧ B", "~A ∧ B", "~A ∧ B", "~A ∧ B");
    test(
        2,
        &[0, 3],
        &[1, 2],
        "(A ∧ B) ∨ (~A ∧ ~B)",
        "(A ∧ B) ∨ (~A ∧ ~B)",
        "(A ∨ ~B) ∧ (~A ∨ B)",
        "(A ∨ ~B) ∧ (~A ∨ B)",
    );

    test(
        3,
        &[4, 6, 7, 1, 2, 3],
        &[5, 0],
        "B ∨ (A ∧ ~C) ∨ (~A ∧ C)",
        "B ∨ (A ∧ ~C) ∨ (~A ∧ C)",
        "(A ∨ B ∨ C) ∧ (~A ∨ B ∨ ~C)",
        "(A ∨ B ∨ C) ∧ (~A ∨ B ∨ ~C)",
    );

    test(
        4,
        &[10, 13, 3, 7, 4],
        &[11, 2, 1, 12, 15, 0, 5, 9, 6],
        "(A ∧ ~B ∧ ~D) ∨ (~A ∧ C ∧ D) ∨ (A ∧ B ∧ ~C ∧ D) ∨ (~A ∧ B ∧ ~C ∧ ~D)",
        "(A ∧ C ∧ ~D) ∨ (~A ∧ C ∧ D) ∨ (A ∧ B ∧ ~C ∧ D) ∨ (~A ∧ B ∧ ~C ∧ ~D)",
        "(B ∨ C) ∧ (A ∨ C ∨ ~D) ∧ (A ∨ ~C ∨ D) ∧ (~A ∨ C ∨ D) ∧ (~A ∨ ~C ∨ ~D)",
        "(B ∨ C) ∧ (A ∨ C ∨ ~D) ∧ (A ∨ ~C ∨ D) ∧ (~A ∨ ~B ∨ D) ∧ (~A ∨ ~C ∨ ~D)",
    );

    test(
        5,
        &[30, 22, 19, 4, 7, 14, 31, 17, 16, 24, 21, 2],
        &[1, 27, 6, 11, 8, 10, 0, 13, 9, 20, 23, 28, 26],
        "(A ∧ ~C ∧ ~D) ∨ (A ∧ ~D ∧ E) ∨ (B ∧ C ∧ D) ∨ (~B ∧ ~C ∧ D) ∨ (A ∧ C ∧ D ∧ ~E) ∨ (~A ∧ ~B ∧ C ∧ ~D) ∨ (~A ∧ ~B ∧ D ∧ E)",
        "(A ∧ ~C ∧ ~D) ∨ (A ∧ ~D ∧ E) ∨ (B ∧ C ∧ D) ∨ (~B ∧ ~C ∧ D) ∨ (A ∧ ~B ∧ D ∧ ~E) ∨ (~A ∧ ~B ∧ C ∧ E) ∨ (~A ∧ C ∧ ~D ∧ ~E)",
        "(A ∨ C ∨ D) ∧ (~B ∨ C ∨ ~D) ∧ (~B ∨ ~C ∨ D) ∧ (~A ∨ ~C ∨ D ∨ E) ∧ (A ∨ B ∨ ~C ∨ ~D ∨ E) ∧ (~A ∨ B ∨ ~C ∨ ~D ∨ ~E)",
        "(A ∨ ~B ∨ D) ∧ (A ∨ C ∨ D) ∧ (~B ∨ C ∨ ~D) ∧ (~A ∨ ~C ∨ D ∨ E) ∧ (A ∨ B ∨ ~C ∨ ~D ∨ E) ∧ (~A ∨ B ∨ ~C ∨ ~D ∨ ~E)"
    );
}

fn test(
    variable_count: u32,
    minterms: &[u32],
    maxterms: &[u32],
    expected_sop: &str,
    expected_sop_all: &str,
    expected_pos: &str,
    expected_pos_all: &str,
) {
    let variables = &qmc::DEFAULT_VARIABLES[..variable_count as usize];
    let dont_cares = Vec::from_iter(get_dont_cares(
        variable_count,
        &HashSet::from_iter(minterms.iter().copied()),
        &HashSet::from_iter(maxterms.iter().copied()),
    ));

    println!(
        "variable_count: {}, minterms: {:?}, maxterms: {:?}, dont_cares: {:?}",
        variable_count, minterms, maxterms, dont_cares
    );

    assert_eq!(
        qmc::minimize(variables, minterms, maxterms, qmc::SOP, false, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_sop,
    );

    assert_eq!(
        qmc::minimize_minterms(variables, minterms, &dont_cares, false, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_sop
    );

    assert_eq!(
        qmc::minimize(variables, minterms, maxterms, qmc::SOP, true, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_sop_all
    );

    assert_eq!(
        qmc::minimize_minterms(variables, minterms, &dont_cares, true, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_sop_all
    );

    assert_eq!(
        qmc::minimize(variables, minterms, maxterms, qmc::POS, false, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_pos
    );

    assert_eq!(
        qmc::minimize_maxterms(variables, maxterms, &dont_cares, false, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_pos
    );

    assert_eq!(
        qmc::minimize(variables, minterms, maxterms, qmc::POS, true, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_pos_all
    );

    assert_eq!(
        qmc::minimize_maxterms(variables, maxterms, &dont_cares, true, None)
            .unwrap()
            .pop()
            .unwrap()
            .to_string(),
        expected_pos_all
    );
}

fn get_dont_cares(
    variable_count: u32,
    minterms: &HashSet<u32>,
    maxterms: &HashSet<u32>,
) -> HashSet<u32> {
    let all_terms: HashSet<u32> = HashSet::from_iter(0..1 << variable_count);
    let cares = minterms.union(maxterms).copied().collect();

    all_terms.difference(&cares).copied().collect()
}
