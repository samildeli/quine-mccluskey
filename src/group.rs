use std::{cell::RefCell, collections::HashSet};

use crate::term::Term;

#[derive(Debug, Clone)]
pub struct Group {
    terms: HashSet<Term>,
    was_combined: RefCell<bool>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            terms: HashSet::new(),
            was_combined: false.into(),
        }
    }

    pub fn group_terms(variable_count: u8, terms: &HashSet<u32>, sop: bool) -> Vec<Group> {
        let mut groups: Vec<Group> = vec![Group::new(); (variable_count + 1) as usize];

        for &term in terms {
            let index = if sop {
                term.count_ones()
            } else {
                term.count_zeros() - (32 - variable_count as u32)
            } as usize;

            groups[index].terms.insert(Term::new(term));
        }

        groups
    }

    pub fn combine(&self, other: &Group) -> Group {
        let mut combined_group = Group::new();

        for term in &self.terms {
            for other_term in &other.terms {
                if let Some(combined_term) = term.combine(other_term) {
                    combined_group.terms.insert(combined_term);
                    *self.was_combined.borrow_mut() = true;
                }
            }
        }

        combined_group
    }

    pub fn get_prime_implicants(&self, dont_cares: &HashSet<u32>) -> Vec<Term> {
        self.terms
            .iter()
            .filter(|term| !term.was_combined() && !term.get_terms().is_subset(dont_cares))
            .cloned()
            .collect()
    }

    pub fn was_combined(&self) -> bool {
        *self.was_combined.borrow()
    }
}
