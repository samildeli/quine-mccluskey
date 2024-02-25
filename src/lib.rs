mod group;
mod implicant;
mod petrick;
mod prime_implicant_chart;
mod solution;

use std::collections::HashSet;

use group::Group;
use implicant::Implicant;
use petrick::Petrick;
use prime_implicant_chart::PrimeImplicantChart;
use solution::Solution;

pub fn minimize<T: AsRef<str>>(
    variables: &[T],
    minterms: &[u32],
    maxterms: &[u32],
    sop: bool,
) -> Vec<Solution> {
    let variables: Vec<_> = variables
        .iter()
        .map(|variable| variable.as_ref().to_string())
        .collect();

    let internal_solutions = minimize_internal(variables.len() as u32, minterms, maxterms, sop);

    internal_solutions
        .into_iter()
        .map(|solution| Solution::new(solution, &variables, sop))
        .collect()
}

pub static DEFAULT_VARIABLES: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];

fn minimize_internal(
    variable_count: u32,
    minterms: &[u32],
    maxterms: &[u32],
    sop: bool,
) -> Vec<Vec<Implicant>> {
    let minterms = HashSet::from_iter(minterms.iter().copied());
    let maxterms = HashSet::from_iter(maxterms.iter().copied());
    let dont_cares = get_dont_cares(variable_count, &minterms, &maxterms);

    let prime_implicants =
        find_prime_implicants(variable_count, &minterms, &maxterms, &dont_cares, sop);
    let mut prime_implicant_chart = PrimeImplicantChart::new(prime_implicants, &dont_cares);
    let essential_prime_implicants = prime_implicant_chart.simplify();
    let petrick_solutions = Petrick::solve(&prime_implicant_chart, variable_count);

    let mut solutions = petrick_solutions
        .iter()
        .map(|solution| [essential_prime_implicants.as_slice(), solution].concat())
        .collect::<Vec<_>>();

    for solution in &mut solutions {
        solution.sort_unstable_by_key(|implicant| implicant.get_literal_count(variable_count));
    }

    solutions
}

fn find_prime_implicants(
    variable_count: u32,
    minterms: &HashSet<u32>,
    maxterms: &HashSet<u32>,
    dont_cares: &HashSet<u32>,
    sop: bool,
) -> Vec<Implicant> {
    let terms = if sop { minterms } else { maxterms };
    let terms = terms.union(dont_cares).copied().collect();

    let mut groups = Group::group_terms(variable_count, &terms, sop);
    let mut prime_implicants = vec![];

    loop {
        let next_groups = (0..groups.len() - 1)
            .map(|i| groups[i].combine(&groups[i + 1]))
            .collect();

        prime_implicants.extend(
            groups
                .iter()
                .flat_map(|group| group.get_prime_implicants(dont_cares)),
        );

        if groups.iter().all(|group| !group.was_combined()) {
            break;
        }

        groups = next_groups;
    }

    prime_implicants
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::Rng;

    use super::*;

    #[test]
    fn test_minimize_exhaustive() {
        for variable_count in 1..=3 {
            let term_combinations = generate_terms_exhaustive(variable_count);

            for terms in &term_combinations {
                for sop in [true, false] {
                    check_and_print_solutions(variable_count, &terms.0, &terms.1, sop);
                }
            }
        }
    }

    #[test]
    fn test_minimize_random() {
        for variable_count in 4..=5 {
            let term_combinations = generate_terms_random(variable_count, 10000);

            for terms in &term_combinations {
                for sop in [true, false] {
                    check_and_print_solutions(variable_count, &terms.0, &terms.1, sop);
                }
            }
        }
    }

    // #[test]
    // fn test_minimize_specific() {
    //     let variable_count = 1;
    //     let sop = true;
    //     let minterms = [];
    //     let maxterms = [];

    //     let all_terms: HashSet<u32> = HashSet::from_iter(0..1 << variable_count);

    //     let maxterms: Vec<_> = all_terms
    //         .difference(&HashSet::from_iter(minterms))
    //         .copied()
    //         .collect();

    //     check_and_print_solutions(
    //         variable_count,
    //         &HashSet::from_iter(minterms),
    //         &HashSet::from_iter(maxterms),
    //         sop,
    //     );
    // }

    #[test]
    fn test_find_prime_implicants() {
        fn test(
            variable_count: u32,
            minterms: &[u32],
            maxterms: &[u32],
            sop: bool,
            answer: &[Implicant],
        ) {
            let minterms = HashSet::from_iter(minterms.iter().copied());
            let maxterms = HashSet::from_iter(maxterms.iter().copied());
            let dont_cares = get_dont_cares(variable_count, &minterms, &maxterms);

            let result =
                find_prime_implicants(variable_count, &minterms, &maxterms, &dont_cares, sop);

            assert_eq!(
                result.into_iter().collect::<HashSet<_>>(),
                HashSet::from_iter(answer.iter().copied())
            );
        }

        test(1, &[], &[0, 1], true, &[]);
        test(1, &[0], &[1], true, &[Implicant::from_str("0")]);
        test(1, &[1], &[0], true, &[Implicant::from_str("1")]);
        test(1, &[0, 1], &[], true, &[Implicant::from_str("-")]);
        test(1, &[], &[], true, &[]);
        test(1, &[], &[0], true, &[]);
        test(1, &[], &[1], true, &[]);
        test(1, &[0], &[], true, &[Implicant::from_str("-")]);
        test(1, &[1], &[], true, &[Implicant::from_str("-")]);

        test(1, &[0, 1], &[], false, &[]);
        test(1, &[1], &[0], false, &[Implicant::from_str("0")]);
        test(1, &[0], &[1], false, &[Implicant::from_str("1")]);
        test(1, &[], &[0, 1], false, &[Implicant::from_str("-")]);
        test(1, &[], &[], false, &[]);
        test(1, &[0], &[], false, &[]);
        test(1, &[1], &[], false, &[]);
        test(1, &[], &[0], false, &[Implicant::from_str("-")]);
        test(1, &[], &[1], false, &[Implicant::from_str("-")]);

        test(
            2,
            &[0, 3],
            &[2],
            true,
            &[Implicant::from_str("0-"), Implicant::from_str("-1")],
        );

        test(
            3,
            &[1, 2, 5],
            &[3, 4, 7],
            true,
            &[
                Implicant::from_str("00-"),
                Implicant::from_str("0-0"),
                Implicant::from_str("-01"),
                Implicant::from_str("-10"),
            ],
        );

        test(
            4,
            &[2, 4, 5, 7, 9],
            &[3, 6, 10, 12, 15],
            true,
            &[
                Implicant::from_str("00-0"),
                Implicant::from_str("01-1"),
                Implicant::from_str("10-1"),
                Implicant::from_str("0-0-"),
                Implicant::from_str("-00-"),
                Implicant::from_str("--01"),
            ],
        );
    }

    fn check_and_print_solutions(
        variable_count: u32,
        minterms: &HashSet<u32>,
        maxterms: &HashSet<u32>,
        sop: bool,
    ) {
        println!(
            "sop: {}, variable_count: {}, minterms: {:?}, maxterms: {:?}",
            sop, variable_count, minterms, maxterms
        );

        let internal_solutions = minimize_internal(
            variable_count,
            &Vec::from_iter(minterms.clone()),
            &Vec::from_iter(maxterms.clone()),
            sop,
        );

        for solution in &internal_solutions {
            assert!(check_solution(minterms, maxterms, sop, solution));
        }

        let variables: Vec<_> = DEFAULT_VARIABLES[..variable_count as usize]
            .iter()
            .map(|variable| variable.to_string())
            .collect();

        let solutions: Vec<_> = internal_solutions
            .into_iter()
            .map(|solution| Solution::new(solution, &variables, sop))
            .collect();

        println!(
            "{:#?}",
            solutions
                .iter()
                .map(|solution| solution.to_string())
                .collect_vec()
        );
    }

    fn check_solution(
        minterms: &HashSet<u32>,
        maxterms: &HashSet<u32>,
        sop: bool,
        solution: &[Implicant],
    ) -> bool {
        let terms = if sop { minterms } else { maxterms };
        let other_terms = if sop { maxterms } else { minterms };
        let covered_terms =
            HashSet::from_iter(solution.iter().flat_map(|implicant| implicant.get_terms()));

        terms.is_subset(&covered_terms) && other_terms.is_disjoint(&covered_terms)
    }

    fn generate_terms_exhaustive(variable_count: u32) -> Vec<(HashSet<u32>, HashSet<u32>)> {
        let mut generated_terms = vec![];
        let all_terms: HashSet<u32> = HashSet::from_iter(0..1 << variable_count);

        for i in 0..=all_terms.len() {
            let minterm_combinations = all_terms
                .iter()
                .copied()
                .combinations(i)
                .map(HashSet::from_iter);

            for minterms in minterm_combinations {
                let other_terms: HashSet<u32> = all_terms.difference(&minterms).copied().collect();

                for j in 0..=other_terms.len() {
                    let maxterm_combinations = other_terms
                        .iter()
                        .copied()
                        .combinations(j)
                        .map(HashSet::from_iter);

                    generated_terms
                        .extend(maxterm_combinations.map(|maxterms| (minterms.clone(), maxterms)));
                }
            }
        }

        generated_terms
    }

    fn generate_terms_random(variable_count: u32, count: u32) -> Vec<(HashSet<u32>, HashSet<u32>)> {
        let mut generated_terms = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            let mut all_terms = Vec::from_iter(0..1 << variable_count);
            let mut minterms = HashSet::new();
            let mut maxterms = HashSet::new();

            for _ in 0..all_terms.len() {
                let term = all_terms.swap_remove(rng.gen_range(0..all_terms.len()));
                let choice = rng.gen_range(1..=3);

                if choice == 1 {
                    minterms.insert(term);
                } else if choice == 2 {
                    maxterms.insert(term);
                }
            }

            generated_terms.push((minterms, maxterms));
        }

        generated_terms
    }
}
