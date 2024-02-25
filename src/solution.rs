use std::fmt::Display;

use crate::implicant::Implicant;

pub struct Solution {
    expression: Vec<Vec<Variable>>,
    sop: bool,
}

impl Solution {
    pub fn new(internal_solution: Vec<Implicant>, variables: &[String], sop: bool) -> Self {
        Solution {
            expression: internal_solution
                .iter()
                .map(|implicant| implicant.to_variables(variables, sop))
                .collect(),
            sop,
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.expression.is_empty() {
            if self.sop {
                return write!(f, "0");
            }

            return write!(f, "1");
        }

        if self.expression[0].is_empty() {
            if self.sop {
                return write!(f, "1");
            }

            return write!(f, "0");
        }

        for (i, variables) in self.expression.iter().enumerate() {
            if self.expression.len() > 1 && variables.len() > 1 {
                write!(f, "(").unwrap();
            }

            for (j, variable) in variables.iter().enumerate() {
                write!(f, "{}", variable).unwrap();

                if j < variables.len() - 1 {
                    write!(f, " {} ", if self.sop { "∧" } else { "∨" }).unwrap();
                }
            }

            if self.expression.len() > 1 && variables.len() > 1 {
                write!(f, ")").unwrap();
            }

            if i < self.expression.len() - 1 {
                write!(f, " {} ", if self.sop { "∨" } else { "∧" }).unwrap();
            }
        }

        Ok(())
    }
}

pub struct Variable {
    name: String,
    is_negated: bool,
}

impl Variable {
    pub fn new(name: String, is_negated: bool) -> Self {
        Variable { name, is_negated }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.is_negated { "~" } else { "" }, self.name)
    }
}
