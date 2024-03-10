use crate::{implicant::Implicant, prime_implicant_chart::PrimeImplicantChart};

pub struct Petrick;

impl Petrick {
    pub fn solve(prime_implicant_chart: &PrimeImplicantChart) -> Vec<Vec<Implicant>> {
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
        Self::filter_minimal_literals(candidates)
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

    fn filter_minimal_literals(candidates: Vec<Vec<Implicant>>) -> Vec<Vec<Implicant>> {
        let get_wildcard_count = |candidate: &Vec<Implicant>| {
            candidate
                .iter()
                .fold(0, |acc, implicant| acc + implicant.wildcard_count())
        };

        let min_count = candidates.iter().map(get_wildcard_count).max().unwrap();

        candidates
            .into_iter()
            .filter(|candidate| get_wildcard_count(candidate) == min_count)
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
    implicants: Vec<Implicant>,
}

impl Product {
    pub fn new(implicant: Implicant) -> Self {
        Product {
            implicants: vec![implicant],
        }
    }

    pub fn and(&self, other: &Self) -> Self {
        let mut implicants = [self.implicants.clone(), other.implicants.clone()].concat();

        implicants.sort_unstable();
        implicants.dedup();

        Product { implicants }
    }

    pub fn absorb(&self, other: &Self) -> Option<Self> {
        if self.is_subset(other) {
            Some(self.clone())
        } else if other.is_subset(self) {
            Some(other.clone())
        } else {
            None
        }
    }

    fn is_subset(&self, other: &Self) -> bool {
        for implicant in self.implicants.iter() {
            let mut found = false;

            for other_implicant in other.implicants.iter() {
                if implicant == other_implicant {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }
}
