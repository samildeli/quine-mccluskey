use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{implicant::Implicant, Form};

/// A minimized boolean expression.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Solution {
    Zero,
    One,
    SOP(Vec<Vec<Variable>>),
    POS(Vec<Vec<Variable>>),
}

impl Solution {
    pub(crate) fn new(internal_solution: &[Implicant], variables: &[String], sop: bool) -> Self {
        let expression = internal_solution
            .iter()
            .map(|implicant| implicant.to_variables(variables, sop))
            .collect::<Vec<_>>();

        let is_zero = if expression.is_empty() {
            sop
        } else if expression[0].is_empty() {
            !sop
        } else {
            false
        };

        let is_one = if expression.is_empty() {
            !sop
        } else if expression[0].is_empty() {
            sop
        } else {
            false
        };

        if is_zero {
            Solution::Zero
        } else if is_one {
            Solution::One
        } else if sop {
            Solution::SOP(expression)
        } else {
            Solution::POS(expression)
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (expression, form) = match self {
            Solution::Zero => return write!(f, "0"),
            Solution::One => return write!(f, "1"),
            Solution::SOP(expression) => (expression, Form::SOP),
            Solution::POS(expression) => (expression, Form::POS),
        };

        for (i, variables) in expression.iter().enumerate() {
            if expression.len() > 1 && variables.len() > 1 {
                write!(f, "(")?;
            }

            for (j, variable) in variables.iter().enumerate() {
                write!(f, "{}", variable)?;

                if j < variables.len() - 1 {
                    write!(f, " {} ", if form == Form::SOP { "∧" } else { "∨" })?;
                }
            }

            if expression.len() > 1 && variables.len() > 1 {
                write!(f, ")")?;
            }

            if i < expression.len() - 1 {
                write!(f, " {} ", if form == Form::SOP { "∨" } else { "∧" })?;
            }
        }

        Ok(())
    }
}

/// A variable as part of a [`Solution`].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Variable {
    pub name: String,
    pub is_negated: bool,
}

impl Variable {
    pub(crate) fn new(name: String, is_negated: bool) -> Self {
        Variable { name, is_negated }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.is_negated { "~" } else { "" }, self.name)
    }
}
