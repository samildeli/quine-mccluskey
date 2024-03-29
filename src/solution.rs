use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{implicant::Implicant, Form};

/// A minimized boolean expression.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Solution {
    One,
    Zero,
    SOP(Vec<Vec<Variable>>),
    POS(Vec<Vec<Variable>>),
}

impl Solution {
    pub(crate) fn new(internal_solution: &[Implicant], variables: &[String], form: Form) -> Self {
        let expression = internal_solution
            .iter()
            .map(|implicant| implicant.to_variables(variables, form))
            .collect::<Vec<_>>();

        let is_one = if expression.is_empty() {
            form == Form::POS
        } else if expression[0].is_empty() {
            form == Form::SOP
        } else {
            false
        };

        let is_zero = if expression.is_empty() {
            form == Form::SOP
        } else if expression[0].is_empty() {
            form == Form::POS
        } else {
            false
        };

        if is_one {
            Solution::One
        } else if is_zero {
            Solution::Zero
        } else if form == Form::SOP {
            Solution::SOP(expression)
        } else {
            Solution::POS(expression)
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (expression, form) = match self {
            Solution::One => return write!(f, "1"),
            Solution::Zero => return write!(f, "0"),
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
