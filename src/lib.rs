mod group;
mod term;

use std::collections::HashSet;

use group::Group;
use term::Term;

pub fn minimize(variable_count: u8, minterms: &[u32], maxterms: &[u32], sop: bool) -> Vec<Term> {
    let minterms = HashSet::from_iter(minterms.iter().copied());
    let maxterms = HashSet::from_iter(maxterms.iter().copied());

    let prime_implicants = find_prime_implicants(variable_count, &minterms, &maxterms, sop);

    vec![]
}

fn find_prime_implicants(
    variable_count: u8,
    minterms: &HashSet<u32>,
    maxterms: &HashSet<u32>,
    sop: bool,
) -> Vec<Term> {
    let dont_cares = get_dont_cares(variable_count, minterms, maxterms);
    let terms = if sop { minterms } else { maxterms };
    let terms = terms.union(&dont_cares).copied().collect();

    let mut groups = Group::group_terms(variable_count, &terms, sop);
    let mut prime_implicants = vec![];

    loop {
        let next_groups = (0..groups.len() - 1)
            .map(|i| groups[i].combine(&groups[i + 1]))
            .collect();

        prime_implicants.extend(
            groups
                .iter()
                .flat_map(|group| group.get_prime_implicants(&dont_cares)),
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
    use std::collections::HashSet;

    use crate::{find_prime_implicants, term::Term};

    #[test]
    fn prime_implicants() {
        fn test(
            variable_count: u8,
            minterms: Vec<u32>,
            maxterms: Vec<u32>,
            sop: bool,
            answer: Vec<Term>,
        ) {
            let minterms = HashSet::from_iter(minterms.iter().copied());
            let maxterms = HashSet::from_iter(maxterms.iter().copied());

            let result = find_prime_implicants(variable_count, &minterms, &maxterms, sop);

            assert_eq!(
                result.into_iter().collect::<HashSet<_>>(),
                HashSet::from_iter(answer)
            );
        }

        test(1, vec![], vec![0, 1], true, vec![]);
        test(1, vec![0], vec![1], true, vec![Term::from("0")]);
        test(1, vec![1], vec![0], true, vec![Term::from("1")]);
        test(1, vec![0, 1], vec![], true, vec![Term::from("-")]);
        test(1, vec![], vec![], true, vec![]);
        test(1, vec![], vec![0], true, vec![]);
        test(1, vec![], vec![1], true, vec![]);
        test(1, vec![0], vec![], true, vec![Term::from("-")]);
        test(1, vec![1], vec![], true, vec![Term::from("-")]);

        test(1, vec![0, 1], vec![], false, vec![]);
        test(1, vec![1], vec![0], false, vec![Term::from("0")]);
        test(1, vec![0], vec![1], false, vec![Term::from("1")]);
        test(1, vec![], vec![0, 1], false, vec![Term::from("-")]);
        test(1, vec![], vec![], false, vec![]);
        test(1, vec![0], vec![], false, vec![]);
        test(1, vec![1], vec![], false, vec![]);
        test(1, vec![], vec![0], false, vec![Term::from("-")]);
        test(1, vec![], vec![1], false, vec![Term::from("-")]);

        test(
            2,
            vec![0, 3],
            vec![2],
            true,
            vec![Term::from("0-"), Term::from("-1")],
        );

        test(
            3,
            vec![1, 2, 5],
            vec![3, 4, 7],
            true,
            vec![
                Term::from("00-"),
                Term::from("0-0"),
                Term::from("-01"),
                Term::from("-10"),
            ],
        );

        test(
            4,
            vec![2, 4, 5, 7, 9],
            vec![3, 6, 10, 12, 15],
            true,
            vec![
                Term::from("00-0"),
                Term::from("01-1"),
                Term::from("10-1"),
                Term::from("0-0-"),
                Term::from("-00-"),
                Term::from("--01"),
            ],
        );
    }
}
