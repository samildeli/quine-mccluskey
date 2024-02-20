use std::collections::HashSet;

use crate::{implicant::Implicant, petrick::Petrick};

pub struct PrimeImplicantChart {
    implicants: Vec<Implicant>,
    table: Vec<Vec<bool>>,
}

impl PrimeImplicantChart {
    pub fn new(implicants: Vec<Implicant>, dont_cares: &HashSet<u32>) -> Self {
        let mut terms = HashSet::new();

        for implicant in &implicants {
            terms.extend(implicant.get_terms());
        }

        terms = terms.difference(dont_cares).copied().collect();

        let mut table = vec![vec![false; implicants.len()]; terms.len()];

        for (y, implicant) in implicants.iter().enumerate() {
            let row_terms = implicant.get_terms();

            for (x, term) in terms.iter().enumerate() {
                if row_terms.contains(term) {
                    table[x][y] = true;
                }
            }
        }

        PrimeImplicantChart { implicants, table }
    }

    pub fn solve(mut self) -> Vec<Vec<Implicant>> {
        let essential_prime_implicants = self.extract_essential_prime_implicants();

        if !self.implicants.is_empty() {
            let petrick_solutions = Petrick::solve(&self);

            petrick_solutions
                .iter()
                .map(|solution| [essential_prime_implicants.as_slice(), solution].concat())
                .collect()
        } else {
            vec![essential_prime_implicants]
        }
    }

    fn extract_essential_prime_implicants(&mut self) -> Vec<Implicant> {
        let mut essential_prime_implicants = vec![];
        let mut rows_to_extract = HashSet::new();
        let mut covered_columns = HashSet::new();

        for col in &self.table {
            let mut marked_count = 0;
            let mut marked_index = usize::MAX;

            for (y, &is_marked) in col.iter().enumerate() {
                if is_marked {
                    marked_count += 1;
                    marked_index = y;
                }

                if marked_count > 1 {
                    break;
                }
            }

            if marked_count == 1 {
                rows_to_extract.insert(marked_index);
                covered_columns.extend(
                    self.table
                        .iter()
                        .enumerate()
                        .filter_map(|(x, col)| col[marked_index].then_some(x)),
                );
            }
        }

        let mut rows_to_extract = Vec::from_iter(rows_to_extract);
        rows_to_extract.sort_unstable();

        for y in rows_to_extract.into_iter().rev() {
            essential_prime_implicants.push(self.implicants.swap_remove(y));

            for col in &mut self.table {
                col.swap_remove(y);
            }
        }

        let mut covered_columns = Vec::from_iter(covered_columns);
        covered_columns.sort_unstable();

        for x in covered_columns.into_iter().rev() {
            self.table.swap_remove(x);
        }

        essential_prime_implicants
    }

    pub fn get_column_covering_implicants(&self) -> Vec<Vec<Implicant>> {
        let mut column_covering_implicants: Vec<Vec<Implicant>> = vec![];

        for x in 0..self.table.len() {
            column_covering_implicants.push(vec![]);

            for (y, implicant) in self.implicants.iter().enumerate() {
                if self.table[x][y] {
                    column_covering_implicants[x].push(implicant.clone());
                }
            }
        }

        column_covering_implicants
    }
}
