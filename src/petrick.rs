use std::collections::HashSet;

use crate::{implicant::Implicant, prime_implicant_chart::PrimeImplicantChart};

pub struct Petrick;

impl Petrick {
    pub fn solve(
        prime_implicant_chart: &PrimeImplicantChart,
        variable_count: u32,
    ) -> Vec<Vec<Implicant>> {
        let mut sums: Vec<SumOfProduct> = prime_implicant_chart
            .get_column_covering_implicants()
            .into_iter()
            .map(SumOfProduct::new)
            .collect();

        if sums.is_empty() {
            return vec![vec![]];
        }

        while sums.len() > 1 {
            #[cfg(test)]
            println!(
                "Distributing {} sums ({} products)...",
                sums.len(),
                sums.iter().fold(0, |acc, sum| acc + sum.products.len())
            );
            Self::distribute(&mut sums);

            #[cfg(test)]
            println!(
                "Absorbing {} sums ({} products)...",
                sums.len(),
                sums.iter().fold(0, |acc, sum| acc + sum.products.len())
            );
            Self::absorb(&mut sums);
        }

        let candidates = sums.pop().unwrap().into();
        let candidates = Self::filter_minimal_implicants(candidates);
        Self::filter_minimal_literals(candidates, variable_count)
    }

    fn distribute(sums: &mut Vec<SumOfProduct>) {
        let mut distributed_sums = vec![];

        for adjacent_sums in sums.chunks_exact(2) {
            distributed_sums.push(adjacent_sums[0].distribute(&adjacent_sums[1]));
        }

        if sums.len() % 2 == 1 {
            distributed_sums.push(sums.pop().unwrap());
        }

        *sums = distributed_sums;
    }

    fn absorb(sums: &mut Vec<SumOfProduct>) {
        for sum in sums {
            sum.absorb();
        }
    }

    fn filter_minimal_implicants(candidates: Vec<Vec<Implicant>>) -> Vec<Vec<Implicant>> {
        let min_count = candidates
            .iter()
            .map(|candidate| candidate.len())
            .min()
            .unwrap();

        candidates
            .into_iter()
            .filter(|candidate| candidate.len() == min_count)
            .collect()
    }

    fn filter_minimal_literals(
        candidates: Vec<Vec<Implicant>>,
        variable_count: u32,
    ) -> Vec<Vec<Implicant>> {
        let get_literal_count = |candidate: &Vec<Implicant>| {
            candidate.iter().fold(0, |acc, implicant| {
                acc + implicant.get_literal_count(variable_count)
            })
        };

        let min_count = candidates.iter().map(get_literal_count).min().unwrap();

        candidates
            .into_iter()
            .filter(|candidate| get_literal_count(candidate) == min_count)
            .collect()
    }
}

struct SumOfProduct {
    products: Vec<Product>,
}

impl SumOfProduct {
    pub fn new(implicants: Vec<Implicant>) -> Self {
        SumOfProduct {
            products: implicants.into_iter().map(Product::new).collect(),
        }
    }

    pub fn distribute(&self, other: &Self) -> Self {
        let mut distributed_products = vec![];

        for product in &self.products {
            for other_product in &other.products {
                distributed_products.push(product.and(other_product));
            }
        }

        SumOfProduct {
            products: distributed_products,
        }
    }

    pub fn absorb(&mut self) {
        for i in (0..self.products.len()).rev() {
            for j in (0..i).rev() {
                if let Some(product) = self.products[i].absorb(&self.products[j]) {
                    self.products[j] = product;
                    self.products.swap_remove(i);
                    break;
                }
            }
        }
    }
}

impl From<SumOfProduct> for Vec<Vec<Implicant>> {
    fn from(value: SumOfProduct) -> Self {
        value
            .products
            .into_iter()
            .map(|product| Vec::from_iter(product.implicants))
            .collect()
    }
}

#[derive(Clone)]
struct Product {
    implicants: HashSet<Implicant>,
}

impl Product {
    pub fn new(implicant: Implicant) -> Self {
        Product {
            implicants: [implicant].into(),
        }
    }

    pub fn and(&self, other: &Self) -> Self {
        Product {
            implicants: self.implicants.union(&other.implicants).cloned().collect(),
        }
    }

    pub fn absorb(&self, other: &Self) -> Option<Self> {
        if self.implicants.is_subset(&other.implicants) {
            Some(self.clone())
        } else if other.implicants.is_subset(&self.implicants) {
            Some(other.clone())
        } else {
            None
        }
    }
}
