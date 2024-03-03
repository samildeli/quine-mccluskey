use quine_mccluskey as qmc;

#[test]
fn solution() {
    test(1, &[], &[0, 1], "0", "0");
    test(1, &[0], &[1], "~A", "~A");
    test(1, &[1], &[0], "A", "A");
    test(1, &[0, 1], &[], "1", "1");
    test(1, &[], &[], "0", "1");
    test(1, &[], &[0], "0", "0");
    test(1, &[], &[1], "0", "0");
    test(1, &[0], &[], "1", "1");
    test(1, &[1], &[], "1", "1");

    test(2, &[], &[], "0", "1");
    test(2, &[1], &[0, 3], "~A ∧ B", "~A ∧ B");
    test(
        2,
        &[0, 3],
        &[1, 2],
        "(A ∧ B) ∨ (~A ∧ ~B)",
        "(A ∨ ~B) ∧ (~A ∨ B)",
    );

    test(
        3,
        &[4, 6, 7, 1, 2, 3],
        &[5, 0],
        "B ∨ (A ∧ ~C) ∨ (~A ∧ C)",
        "(A ∨ B ∨ C) ∧ (~A ∨ B ∨ ~C)",
    );

    test(
        4,
        &[10, 13, 3, 7, 4],
        &[11, 2, 1, 12, 15, 0, 5, 9, 6],
        "(A ∧ ~B ∧ ~D) ∨ (~A ∧ C ∧ D) ∨ (A ∧ B ∧ ~C ∧ D) ∨ (~A ∧ B ∧ ~C ∧ ~D)",
        "(B ∨ C) ∧ (A ∨ C ∨ ~D) ∧ (A ∨ ~C ∨ D) ∧ (~A ∨ ~B ∨ D) ∧ (~A ∨ ~C ∨ ~D)",
    );
}

fn test(
    variable_count: usize,
    minterms: &[u32],
    maxterms: &[u32],
    expected_sop: &str,
    expected_pos: &str,
) {
    assert_eq!(
        qmc::minimize(
            &qmc::DEFAULT_VARIABLES[..variable_count],
            minterms,
            maxterms,
            true,
            None
        )
        .unwrap()[0]
            .to_string(),
        expected_sop
    );

    assert_eq!(
        qmc::minimize(
            &qmc::DEFAULT_VARIABLES[..variable_count],
            minterms,
            maxterms,
            false,
            None
        )
        .unwrap()[0]
            .to_string(),
        expected_pos
    );
}
