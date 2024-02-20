use std::{cell::RefCell, collections::HashSet};

use crate::implicant::Implicant;

#[derive(Debug, Clone)]
pub struct Group {
    implicants: HashSet<Implicant>,
    was_combined: RefCell<bool>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            implicants: HashSet::new(),
            was_combined: false.into(),
        }
    }

    pub fn group_terms(variable_count: u8, terms: &HashSet<u32>, sop: bool) -> Vec<Self> {
        let mut groups = vec![Group::new(); (variable_count + 1) as usize];

        for &term in terms {
            let index = if sop {
                term.count_ones()
            } else {
                term.count_zeros() - (32 - variable_count as u32)
            } as usize;

            groups[index]
                .implicants
                .insert(Implicant::new(variable_count, term));
        }

        groups
    }

    pub fn combine(&self, other: &Self) -> Self {
        let mut combined_group = Group::new();

        for implicant in &self.implicants {
            for other_implicant in &other.implicants {
                if let Some(combined_implicant) = implicant.combine(other_implicant) {
                    combined_group.implicants.insert(combined_implicant);
                    *self.was_combined.borrow_mut() = true;
                }
            }
        }

        combined_group
    }

    pub fn get_prime_implicants(&self, dont_cares: &HashSet<u32>) -> Vec<Implicant> {
        self.implicants
            .iter()
            .filter(|implicant| {
                !implicant.was_combined() && !implicant.get_terms().is_subset(dont_cares)
            })
            .cloned()
            .collect()
    }

    pub fn was_combined(&self) -> bool {
        *self.was_combined.borrow()
    }
}
