//! # Data models
//! This module holds data models of the project.

/// # Category
/// Those are the different categories supported
#[derive(PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let trans = Transaction::new("Hamburguer".to_string(), 1.64, Category::Food);

        assert_eq!(&trans.description, "Hamburguer");
    }
}
