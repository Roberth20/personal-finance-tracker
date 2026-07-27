//! # Data models
//! This module holds data models of the project.
use std::str::FromStr;

/// # Category
/// Those are the different categories supported
#[derive(Debug, PartialEq)]
pub enum Category {
    Food,
    Rent,
    Salary,
    Services,
    Expenses,
    Credit,
}

/// # Transaction
/// Data type with simple metadata
pub struct Transaction {
    pub description: String,
    pub amount: f64,
    pub category: Category,
}

impl Transaction {
    /// Create a Transaction objects
    pub fn new(description: String, amount: f64, category: Category) -> Self {
        Self {
            description,
            amount,
            category,
        }
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_category(&self) -> &Category {
        &self.category
    }
}

/// Possible states of the workflow.
#[derive(Debug, PartialEq)]
pub enum FlowState {
    Unknown,
    AddTransaction,
    CalculateTotal,
    FilterCategory,
    Exit,
}

impl FromStr for FlowState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Unknown),
            "1" => Ok(Self::AddTransaction),
            "2" => Ok(Self::CalculateTotal),
            "3" => Ok(Self::FilterCategory),
            "4" => Ok(Self::Exit),
            _ => Err(format!(
                "{} not a valid input. It must be one of (1, 2, 3, 4)",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let trans = Transaction::new("Hamburguer".to_string(), 1.64, Category::Food);

        assert_eq!(&trans.description, "Hamburguer");
    }
}
