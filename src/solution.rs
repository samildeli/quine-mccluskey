use std::fmt::Display;

use crate::implicant::Implicant;

#[derive(Debug)]
pub struct Solution {
    expression: Vec<Vec<Variable>>,
    sop: bool,
}

impl Solution {
    pub(crate) fn new(internal_solution: &[Implicant], variables: &[String], sop: bool) -> Self {
        Solution {
            expression: internal_solution
                .iter()
                .map(|implicant| implicant.to_variables(variables, sop))
                .collect(),
            sop,
        }
    }

    pub fn expression(&self) -> &Vec<Vec<Variable>> {
        &self.expression
    }

    pub fn sop(&self) -> bool {
        self.sop
    }

    pub fn is_zero(&self) -> bool {
        if self.expression.is_empty() {
            self.sop
        } else if self.expression[0].is_empty() {
            !self.sop
        } else {
            false
        }
    }

    pub fn is_one(&self) -> bool {
        if self.expression.is_empty() {
            !self.sop
        } else if self.expression[0].is_empty() {
            self.sop
        } else {
            false
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        if self.is_one() {
            return write!(f, "1");
        }

        for (i, variables) in self.expression.iter().enumerate() {
            if self.expression.len() > 1 && variables.len() > 1 {
                write!(f, "(")?;
            }

            for (j, variable) in variables.iter().enumerate() {
                write!(f, "{}", variable)?;

                if j < variables.len() - 1 {
                    write!(f, " {} ", if self.sop { "∧" } else { "∨" })?;
                }
            }

            if self.expression.len() > 1 && variables.len() > 1 {
                write!(f, ")")?;
            }

            if i < self.expression.len() - 1 {
                write!(f, " {} ", if self.sop { "∨" } else { "∧" })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Variable {
    name: String,
    is_negated: bool,
}

impl Variable {
    pub(crate) fn new(name: String, is_negated: bool) -> Self {
        Variable { name, is_negated }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_negated(&self) -> bool {
        self.is_negated
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.is_negated { "~" } else { "" }, self.name)
    }
}
