mod group;
mod implicant;
mod petrick;
mod prime_implicant_chart;

use std::collections::HashSet;

use group::Group;
use implicant::Implicant;
use prime_implicant_chart::PrimeImplicantChart;

pub fn minimize(
    variable_count: u8,
    minterms: &[u32],
    maxterms: &[u32],
    sop: bool,
) -> Vec<Vec<String>> {
    vec![]
}

fn minimize_internal(
    variable_count: u8,
    minterms: &[u32],
    maxterms: &[u32],
    sop: bool,
) -> Vec<Vec<Implicant>> {
    let minterms = HashSet::from_iter(minterms.iter().copied());
    let maxterms = HashSet::from_iter(maxterms.iter().copied());
    let dont_cares = get_dont_cares(variable_count, &minterms, &maxterms);

    let prime_implicants =
        find_prime_implicants(variable_count, &minterms, &maxterms, &dont_cares, sop);
    let prime_implicant_chart = PrimeImplicantChart::new(prime_implicants, &dont_cares);
    prime_implicant_chart.solve()
}

fn find_prime_implicants(
    variable_count: u8,
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
    variable_count: u8,
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
    fn test_minimize_internal_exhaustive() {
        test_minimize_internal(1..=3, None);
    }

    #[test]
    fn test_minimize_internal_random() {
        test_minimize_internal(1..=15, Some(1));
    }

    fn test_minimize_internal<R: Iterator<Item = u8>>(variable_count_range: R, count: Option<u32>) {
        for variable_count in variable_count_range {
            let term_combinations = if let Some(count) = count {
                generate_terms_random(variable_count, count)
            } else {
                generate_terms_exhaustive(variable_count)
            };

            for sop in [true, false] {
                for terms in &term_combinations {
                    println!(
                        "sop: {}, variable_count: {}, minterms: {:?}, maxterms: {:?}",
                        sop, variable_count, terms.0, terms.1
                    );

                    let solutions = minimize_internal(
                        variable_count,
                        &Vec::from_iter(terms.0.clone()),
                        &Vec::from_iter(terms.1.clone()),
                        sop,
                    );

                    for solution in &solutions {
                        assert!(check_solution(&terms.0, &terms.1, sop, solution));
                    }
                }
            }
        }
    }

    #[test]
    fn test_find_prime_implicants() {
        fn test(
            variable_count: u8,
            minterms: Vec<u32>,
            maxterms: Vec<u32>,
            sop: bool,
            answer: Vec<Implicant>,
        ) {
            let minterms = HashSet::from_iter(minterms.iter().copied());
            let maxterms = HashSet::from_iter(maxterms.iter().copied());
            let dont_cares = get_dont_cares(variable_count, &minterms, &maxterms);

            let result =
                find_prime_implicants(variable_count, &minterms, &maxterms, &dont_cares, sop);

            assert_eq!(
                result.into_iter().collect::<HashSet<_>>(),
                HashSet::from_iter(answer)
            );
        }

        test(1, vec![], vec![0, 1], true, vec![]);
        test(1, vec![0], vec![1], true, vec![Implicant::from("0")]);
        test(1, vec![1], vec![0], true, vec![Implicant::from("1")]);
        test(1, vec![0, 1], vec![], true, vec![Implicant::from("-")]);
        test(1, vec![], vec![], true, vec![]);
        test(1, vec![], vec![0], true, vec![]);
        test(1, vec![], vec![1], true, vec![]);
        test(1, vec![0], vec![], true, vec![Implicant::from("-")]);
        test(1, vec![1], vec![], true, vec![Implicant::from("-")]);

        test(1, vec![0, 1], vec![], false, vec![]);
        test(1, vec![1], vec![0], false, vec![Implicant::from("0")]);
        test(1, vec![0], vec![1], false, vec![Implicant::from("1")]);
        test(1, vec![], vec![0, 1], false, vec![Implicant::from("-")]);
        test(1, vec![], vec![], false, vec![]);
        test(1, vec![0], vec![], false, vec![]);
        test(1, vec![1], vec![], false, vec![]);
        test(1, vec![], vec![0], false, vec![Implicant::from("-")]);
        test(1, vec![], vec![1], false, vec![Implicant::from("-")]);

        test(
            2,
            vec![0, 3],
            vec![2],
            true,
            vec![Implicant::from("0-"), Implicant::from("-1")],
        );

        test(
            3,
            vec![1, 2, 5],
            vec![3, 4, 7],
            true,
            vec![
                Implicant::from("00-"),
                Implicant::from("0-0"),
                Implicant::from("-01"),
                Implicant::from("-10"),
            ],
        );

        test(
            4,
            vec![2, 4, 5, 7, 9],
            vec![3, 6, 10, 12, 15],
            true,
            vec![
                Implicant::from("00-0"),
                Implicant::from("01-1"),
                Implicant::from("10-1"),
                Implicant::from("0-0-"),
                Implicant::from("-00-"),
                Implicant::from("--01"),
            ],
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
        let mut covered_terms = HashSet::new();

        for implicant in solution {
            covered_terms.extend(implicant.get_terms());
        }

        terms.is_subset(&covered_terms) && other_terms.is_disjoint(&covered_terms)
    }

    fn generate_terms_exhaustive(variable_count: u8) -> Vec<(HashSet<u32>, HashSet<u32>)> {
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

    fn generate_terms_random(variable_count: u8, count: u32) -> Vec<(HashSet<u32>, HashSet<u32>)> {
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
