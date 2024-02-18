use std::{cell::RefCell, collections::HashSet, hash::Hash};

#[derive(Debug, Clone)]
pub struct Term {
    value: u32,
    mask: u32,
    was_combined: RefCell<bool>,
}

impl Term {
    pub fn new(value: u32) -> Term {
        Term {
            value,
            mask: 0,
            was_combined: false.into(),
        }
    }

    pub fn combine(&self, other: &Term) -> Option<Term> {
        if self.mask == other.mask {
            let diff = self.value ^ other.value;

            if diff.count_ones() == 1 {
                *self.was_combined.borrow_mut() = true;
                *other.was_combined.borrow_mut() = true;

                Some(Term {
                    value: self.value & !diff,
                    mask: self.mask | diff,
                    was_combined: false.into(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_terms(&self) -> HashSet<u32> {
        fn get_terms_(value: u32, mask: u32, terms: &mut HashSet<u32>) {
            let one_pos = mask.trailing_zeros();

            if one_pos < 32 {
                let mask = mask & !(1 << one_pos);
                get_terms_(value, mask, terms);
                get_terms_(value | (1 << one_pos), mask, terms);
            } else {
                terms.insert(value);
            }
        }

        let mut terms = HashSet::new();
        get_terms_(self.value, self.mask, &mut terms);
        terms
    }

    pub fn was_combined(&self) -> bool {
        *self.was_combined.borrow()
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.mask == other.mask
    }
}

impl Eq for Term {}

impl Hash for Term {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.mask.hash(state);
    }
}

impl From<&str> for Term {
    fn from(value: &str) -> Self {
        Term {
            value: u32::from_str_radix(&value.replace('-', "0"), 2).unwrap(),
            mask: u32::from_str_radix(&value.replace('1', "0").replace('-', "1"), 2).unwrap(),
            was_combined: false.into(),
        }
    }
}
