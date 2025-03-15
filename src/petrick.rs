use crate::timeout_signal::TTimeoutSignal;
use crate::{implicant::Implicant, prime_implicant_chart::PrimeImplicantChart, Error};

pub struct Petrick;

impl Petrick {
    pub fn solve(
        prime_implicant_chart: &PrimeImplicantChart,
        timeout_signal: &impl TTimeoutSignal,
    ) -> Result<Vec<Vec<Implicant>>, Error> {
        let mut sums: Vec<SumOfProduct> = prime_implicant_chart
            .get_column_covering_implicants()
            .into_iter()
            .map(SumOfProduct::new)
            .collect();

        if sums.is_empty() {
            return Ok(vec![vec![]]);
        }

        while sums.len() > 1 && timeout_signal.is_not_signaled() {
            #[cfg(test)]
            println!(
                "Distributing {} sums ({} products)...",
                sums.len(),
                sums.iter().fold(0, |acc, sum| acc + sum.products.len())
            );
            Self::distribute(&mut sums, timeout_signal)?;

            #[cfg(test)]
            println!(
                "Absorbing {} sums ({} products)...",
                sums.len(),
                sums.iter().fold(0, |acc, sum| acc + sum.products.len())
            );
            Self::absorb(&mut sums, timeout_signal)?;
        }

        if timeout_signal.is_signaled() {
            Err(Error::Timeout)
        } else {
            let candidates = sums.pop().unwrap().into();
            let candidates = Self::filter_minimal_implicants(candidates);
            Ok(Self::filter_minimal_literals(candidates))
        }
    }

    fn distribute(
        sums: &mut Vec<SumOfProduct>,
        timeout_signal: &impl TTimeoutSignal,
    ) -> Result<(), Error> {
        const CHUNK_SIZE: usize = 2;

        let mut distributed_sums = Vec::with_capacity((sums.len() + (CHUNK_SIZE - 1)) / CHUNK_SIZE);

        for adjacent_sums in sums.chunks_exact(CHUNK_SIZE) {
            if timeout_signal.is_signaled() {
                return Err(Error::Timeout);
            }

            distributed_sums.push(adjacent_sums[0].distribute(&adjacent_sums[1], timeout_signal)?);
        }

        if sums.len() % 2 == 1 {
            distributed_sums.push(sums.pop().unwrap());
        }

        *sums = distributed_sums;

        if timeout_signal.is_signaled() {
            Err(Error::Timeout)
        } else {
            Ok(())
        }
    }

    fn absorb(
        sums: &mut Vec<SumOfProduct>,
        timeout_signal: &impl TTimeoutSignal,
    ) -> Result<(), Error> {
        for sum in sums {
            sum.absorb(timeout_signal)?;
        }

        if timeout_signal.is_signaled() {
            Err(Error::Timeout)
        } else {
            Ok(())
        }
    }

    fn filter_minimal_implicants(candidates: Vec<Vec<Implicant>>) -> Vec<Vec<Implicant>> {
        let min_count = candidates.iter().map(Vec::len).min().unwrap();

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

    pub fn distribute(
        &self,
        other: &Self,
        timeout_signal: &impl TTimeoutSignal,
    ) -> Result<Self, Error> {
        let mut distributed_products =
            Vec::with_capacity(self.products.len() * other.products.len());

        for product in &self.products {
            if timeout_signal.is_signaled() {
                return Err(Error::Timeout);
            }

            for other_product in &other.products {
                distributed_products.push(product.and(other_product));
            }
        }

        if timeout_signal.is_signaled() {
            Err(Error::Timeout)
        } else {
            Ok(SumOfProduct {
                products: distributed_products,
            })
        }
    }

    pub fn absorb(&mut self, timeout_signal: &impl TTimeoutSignal) -> Result<(), Error> {
        for i in (0..self.products.len()).rev() {
            if timeout_signal.is_signaled() {
                return Err(Error::Timeout);
            }

            for j in (0..i).rev() {
                if let Some(product) = self.products[i].absorb(&self.products[j]) {
                    self.products[j] = product;
                    self.products.swap_remove(i);
                    break;
                }
            }
        }

        if timeout_signal.is_signaled() {
            Err(Error::Timeout)
        } else {
            Ok(())
        }
    }
}

impl From<SumOfProduct> for Vec<Vec<Implicant>> {
    fn from(value: SumOfProduct) -> Self {
        value
            .products
            .into_iter()
            .map(|product| product.implicants)
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
