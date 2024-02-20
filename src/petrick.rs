use std::collections::HashSet;

use crate::{prime_implicant_chart::PrimeImplicantChart, term::Term};

pub struct Petrick;

impl Petrick {
    pub fn solve(prime_implicant_chart: &PrimeImplicantChart) -> Vec<Vec<Term>> {
        let mut sums: Vec<SumOfProduct> = prime_implicant_chart
            .get_column_covering_implicants()
            .into_iter()
            .map(SumOfProduct::new)
            .collect();

        while sums.len() > 1 {
            Self::distribute(&mut sums);
            Self::absorb(&mut sums);
        }

        let candidates = sums.pop().unwrap().into();
        let candidates = Self::filter_minimal_implicants(candidates);
        Self::filter_minimal_literals(candidates)
    }

    fn distribute(sums: &mut Vec<SumOfProduct>) {
        let mut new_sums = vec![];

        for i in (0..sums.len()).step_by(2) {
            if i < sums.len() - 1 {
                new_sums.push(sums[i].distribute(&sums[i + 1]));
            } else {
                new_sums.push(sums.pop().unwrap());
            }
        }

        *sums = new_sums;
    }

    fn absorb(sums: &mut Vec<SumOfProduct>) {
        for sum in sums {
            sum.absorb();
        }
    }

    fn filter_minimal_implicants(candidates: Vec<Vec<Term>>) -> Vec<Vec<Term>> {
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

    fn filter_minimal_literals(candidates: Vec<Vec<Term>>) -> Vec<Vec<Term>> {
        let get_literal_count = |candidate: &Vec<Term>| {
            candidate
                .iter()
                .fold(0, |acc, implicant| acc + implicant.get_literal_count())
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
    pub fn new(terms: Vec<Term>) -> Self {
        SumOfProduct {
            products: terms.into_iter().map(Product::new).collect(),
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
            for j in (0..self.products.len() - 1).rev() {
                if let Some(product) = self.products[i].absorb(&self.products[j]) {
                    self.products[j] = product;
                    self.products.pop();
                    break;
                }
            }
        }
    }
}

impl From<SumOfProduct> for Vec<Vec<Term>> {
    fn from(value: SumOfProduct) -> Self {
        value
            .products
            .into_iter()
            .map(|product| Vec::from_iter(product.terms))
            .collect()
    }
}

#[derive(Clone)]
struct Product {
    terms: HashSet<Term>,
}

impl Product {
    pub fn new(term: Term) -> Self {
        Product {
            terms: [term].into(),
        }
    }

    pub fn and(&self, other: &Self) -> Self {
        Product {
            terms: self.terms.union(&other.terms).cloned().collect(),
        }
    }

    pub fn absorb(&self, other: &Self) -> Option<Self> {
        if self.terms.is_subset(&other.terms) {
            Some(self.clone())
        } else if other.terms.is_subset(&self.terms) {
            Some(other.clone())
        } else {
            None
        }
    }
}
