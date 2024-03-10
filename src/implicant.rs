use std::{cmp::Ordering, collections::HashSet, hash::Hash};

use crate::solution::Variable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Implicant {
    value: u32,
    mask: u32,
}

impl Implicant {
    pub fn new(term: u32) -> Self {
        Implicant {
            value: term,
            mask: 0,
        }
    }

    pub fn combine(&self, other: Self) -> Option<Self> {
        if self.mask == other.mask {
            let diff = self.value ^ other.value;

            if diff.count_ones() == 1 {
                Some(Implicant {
                    value: self.value & !diff,
                    mask: self.mask | diff,
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
            let wildcard_index = mask.trailing_zeros();

            if wildcard_index < 32 {
                let mask = mask & !(1 << wildcard_index);

                get_terms_(value, mask, terms);
                get_terms_(value | 1 << wildcard_index, mask, terms);
            } else {
                terms.insert(value);
            }
        }

        let mut terms = HashSet::new();

        get_terms_(self.value, self.mask, &mut terms);

        terms
    }

    pub fn wildcard_count(&self) -> u32 {
        self.mask.count_ones()
    }

    pub fn to_variables(self, variable_names: &[String], sop: bool) -> Vec<Variable> {
        let mut variables = vec![];
        let variable_count = variable_names.len();

        for i in (0..variable_count).rev() {
            let value_bit = self.value >> i & 1;
            let mask_bit = self.mask >> i & 1;

            if mask_bit != 1 {
                let index = variable_count - i - 1;
                let is_negated = sop && value_bit == 0 || !sop && value_bit == 1;

                variables.push(Variable::new(variable_names[index].clone(), is_negated));
            }
        }

        variables
    }
}

pub trait VariableSort {
    fn variable_sort(&mut self, sop: bool);
}

impl VariableSort for Vec<Implicant> {
    fn variable_sort(&mut self, sop: bool) {
        self.sort_unstable_by(|impl1, impl2| {
            let ordering = impl2.mask.count_ones().cmp(&impl1.mask.count_ones());

            if ordering != Ordering::Equal {
                return ordering;
            }

            for i in (0..32).rev() {
                let value_bit1 = impl1.value >> i & 1;
                let value_bit2 = impl2.value >> i & 1;
                let mask_bit1 = impl1.mask >> i & 1;
                let mask_bit2 = impl2.mask >> i & 1;

                // If both bits are the same variable but one is negated and the other is not,
                if mask_bit1 == 0 && mask_bit2 == 0 && value_bit1 != value_bit2 {
                    // put the implicant with the non-negated variable before.
                    if sop && value_bit1 == 1 && value_bit2 == 0
                        || !sop && value_bit1 == 0 && value_bit2 == 1
                    {
                        return Ordering::Less;
                    }

                    return Ordering::Greater;
                }

                // If only one of them is a variable, put the implicant with the variable before.
                let ordering = mask_bit1.cmp(&mask_bit2);

                if ordering != Ordering::Equal {
                    return ordering;
                }
            }

            Ordering::Equal
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Implicant {
        pub fn from_str(str: &str) -> Self {
            Implicant {
                value: u32::from_str_radix(&str.replace('-', "0"), 2).unwrap(),
                mask: u32::from_str_radix(&str.replace('1', "0").replace('-', "1"), 2).unwrap(),
            }
        }

        pub fn to_str(self, variable_count: u32) -> String {
            let mut str = String::new();

            for i in (0..variable_count).rev() {
                let value_bit = self.value >> i & 1;
                let mask_bit = self.mask >> i & 1;

                if mask_bit == 1 {
                    str.push('-');
                } else {
                    str.push(if value_bit == 1 { '1' } else { '0' });
                }
            }

            str
        }
    }
}
