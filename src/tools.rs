//! # Tools
//! This module contains the functions and tools of the project. Those are
//! isolated runtimes to simplify the tasks.
//!
//! The tools are:
//! * `prompt` to capture user input.
//! * `add_transaction` to grow order book.
//! * `calculate_total_balance` to get totals.
//! * `filter_by_category` list items of one category.
use crate::models;
use std::io::Write;

/// This function print a message prompt in the terminal and capture
/// the input from the user.
pub fn prompt(text: &str) -> Result<String, std::io::Error> {
    print!("{} ", text);
    std::io::stdout().flush()?;

    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;

    Ok(response.trim_end().to_string())
}

/// Add a new transaction to transaction book in memory.
pub fn add_transaction(
    transaction: models::Transaction,
    transactions: &mut Vec<models::Transaction>,
) {
    transactions.push(transaction);
}

/// Iterate over the transactions sum up the amounts.
pub fn calculate_total_balance(transactions: &[models::Transaction]) -> f64 {
    transactions.iter().map(|t| t.get_amount()).sum()
}

/// Return the transaction history with only the desired category
pub fn filter_by_category(category: models::Category, transactions: &Vec<models::Transaction>) {
    for t in transactions {
        if &category == t.get_category() {
            println!("Transaction: {}, with amount: {}", &t.description, t.amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_transactions() -> Vec<models::Transaction> {
        let mut trans: Vec<models::Transaction> = Vec::new();

        add_transaction(
            models::Transaction::new("Coffee".to_string(), -0.5, models::Category::Food),
            &mut trans,
        );

        add_transaction(
            models::Transaction::new("Bonus".to_string(), 1.0, models::Category::Salary),
            &mut trans,
        );

        trans
    }

    #[test]
    fn test_add_transaction() {
        let mut empty_vec: Vec<models::Transaction> = Vec::new();
        let transaction =
            models::Transaction::new("Coffee".to_string(), 0.5, models::Category::Food);

        assert_eq!(empty_vec.len(), 0);

        add_transaction(transaction, &mut empty_vec);

        assert_eq!(empty_vec.len(), 1);
    }

    #[test]
    fn test_calculate_total() {
        let trans = sample_transactions();

        assert_eq!(calculate_total_balance(&trans), 0.5);
    }
}
