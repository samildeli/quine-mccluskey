// See the paper "Minimization of Boolean expressions using matrix algebra"

use crate::implicant::Implicant;
use crate::timeout_signal::TTimeoutSignal;
use crate::Error;
use std::collections::{HashMap, HashSet};

pub struct PrimeImplicantChart {
    implicants: Vec<Implicant>,
    rows: Vec<Vec<bool>>,
    terms: Vec<u32>,
    cols: Vec<Vec<bool>>,
    essential_prime_implicants: Vec<Implicant>,
}

impl PrimeImplicantChart {
    pub fn new(implicants: Vec<Implicant>, dont_cares: &HashSet<u32>) -> Self {
        let mut terms = HashSet::new();

        for implicant in &implicants {
            terms.extend(implicant.get_terms());
        }

        terms = terms.difference(dont_cares).copied().collect();

        let mut rows = vec![vec![false; terms.len()]; implicants.len()];
        let mut cols = vec![vec![false; implicants.len()]; terms.len()];

        let term_indices: HashMap<u32, usize> =
            HashMap::from_iter(terms.iter().enumerate().map(|(i, &term)| (term, i)));

        for (y, implicant) in implicants.iter().enumerate() {
            let row_terms = implicant.get_terms();
            let row_terms = row_terms.difference(dont_cares);

            for term in row_terms {
                let x = *term_indices.get(term).unwrap();
                rows[y][x] = true;
                cols[x][y] = true;
            }
        }

        PrimeImplicantChart {
            implicants,
            rows,
            terms: Vec::from_iter(terms),
            cols,
            essential_prime_implicants: vec![],
        }
    }

    pub fn simplify(
        &mut self,
        timeout_signal: &impl TTimeoutSignal,
        only_extract: bool,
    ) -> Result<Vec<Implicant>, Error> {
        #[cfg(test)]
        println!(
            "Simplifying {} implicants and {} terms",
            self.implicants.len(),
            self.terms.len()
        );

        self.sort();

        if only_extract {
            self.extract_essential_prime_implicants();
            return Ok(self.essential_prime_implicants.clone());
        }

        while timeout_signal.is_not_signaled() {
            let any_essentials_extracted = self.extract_essential_prime_implicants();
            let any_terms_removed = self.remove_dominating_terms();
            let any_implicants_removed = self.remove_dominated_implicants();

            if !any_essentials_extracted && !any_terms_removed && !any_implicants_removed {
                break;
            }
        }

        if timeout_signal.is_signaled() {
            Err(Error::Timeout)
        } else {
            Ok(self.essential_prime_implicants.clone())
        }
    }

    pub fn get_column_covering_implicants(&self) -> Vec<Vec<Implicant>> {
        let mut column_covering_implicants = Vec::with_capacity(self.terms.len());

        for x in 0..self.terms.len() {
            column_covering_implicants.push(
                self.implicants
                    .iter()
                    .enumerate()
                    .filter_map(|(y, &implicant)| self.cols[x][y].then_some(implicant))
                    .collect(),
            );
        }

        column_covering_implicants
    }

    fn extract_essential_prime_implicants(&mut self) -> bool {
        let mut rows_to_extract = HashSet::new();
        let mut covered_columns = HashSet::new();

        for col in &self.cols {
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
                    self.cols
                        .iter()
                        .enumerate()
                        .filter_map(|(x, col)| col[marked_index].then_some(x)),
                );
            }
        }

        let mut rows_to_extract = Vec::from_iter(rows_to_extract);
        rows_to_extract.sort_unstable();

        let mut extracted_implicants = Vec::with_capacity(rows_to_extract.len());
        for y in rows_to_extract.into_iter().rev() {
            extracted_implicants.push(self.remove_row(y));
        }

        let mut covered_columns = Vec::from_iter(covered_columns);
        covered_columns.sort_unstable();

        for &x in covered_columns.iter().rev() {
            self.remove_col(x);
        }

        #[cfg(test)]
        println!(
            "Extracted {} implicants and removed {} terms",
            extracted_implicants.len(),
            covered_columns.len()
        );

        let any_extracted = !extracted_implicants.is_empty();

        self.essential_prime_implicants.extend(extracted_implicants);

        any_extracted
    }

    fn remove_dominating_terms(&mut self) -> bool {
        let mut removed = false;
        #[cfg(test)]
        let mut count = 0;

        for x1 in (0..self.terms.len()).rev() {
            for x2 in (0..self.terms.len()).rev().filter(|&x| x != x1) {
                if is_dominating(&self.cols[x1], &self.cols[x2]) {
                    self.remove_col(x1);
                    removed = true;
                    #[cfg(test)]
                    {
                        count += 1;
                    }
                    break;
                }
            }
        }

        #[cfg(test)]
        println!("Removed {} terms", count);

        removed
    }

    fn remove_dominated_implicants(&mut self) -> bool {
        let mut removed = false;
        #[cfg(test)]
        let mut count = 0;

        for y1 in (0..self.implicants.len()).rev() {
            for y2 in (0..self.implicants.len()).rev().filter(|&y| y != y1) {
                if is_dominating(&self.rows[y2], &self.rows[y1])
                    // Only remove if it has more or an equal number of literals.
                    && self.implicants[y1].wildcard_count() <= self.implicants[y2].wildcard_count()
                {
                    self.remove_row(y1);
                    removed = true;
                    #[cfg(test)]
                    {
                        count += 1;
                    }
                    break;
                }
            }
        }

        #[cfg(test)]
        println!("Removed {} implicants", count);

        removed
    }

    fn sort(&mut self) {
        // Sort implicants to make the simplification deterministic.
        let mut sorted_implicants: Vec<_> = self.implicants.iter().zip(self.rows.clone()).collect();
        sorted_implicants.sort_unstable_by(|(impl1, _), (impl2, _)| impl1.cmp(impl2));

        (self.implicants, self.rows) = sorted_implicants.into_iter().unzip();

        let mut new_cols = Vec::with_capacity(self.terms.len());

        for x in 0..self.terms.len() {
            new_cols.push(self.rows.iter().map(|row| row[x]).collect());
        }

        self.cols = new_cols;

        // Sorting terms makes absorption more effective in petrick.
        let mut sorted_terms: Vec<_> = self.terms.iter().zip(self.cols.clone()).collect();
        sorted_terms.sort_unstable_by(|(term1, _), (term2, _)| term1.cmp(term2));

        (self.terms, self.cols) = sorted_terms.into_iter().unzip();

        let mut new_rows = Vec::with_capacity(self.implicants.len());

        for y in 0..self.implicants.len() {
            new_rows.push(self.cols.iter().map(|col| col[y]).collect());
        }

        self.rows = new_rows;
    }

    fn remove_row(&mut self, y: usize) -> Implicant {
        self.rows.swap_remove(y);

        for col in &mut self.cols {
            col.swap_remove(y);
        }

        self.implicants.swap_remove(y)
    }

    fn remove_col(&mut self, x: usize) -> u32 {
        self.cols.swap_remove(x);

        for row in &mut self.rows {
            row.swap_remove(x);
        }

        self.terms.swap_remove(x)
    }
}

fn is_dominating(marks: &[bool], other_marks: &[bool]) -> bool {
    marks
        .iter()
        .zip(other_marks)
        .all(|(&mark, other_mark)| !other_mark || mark)
}
