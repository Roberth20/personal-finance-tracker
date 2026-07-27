use crate::models;

pub fn add_transaction(
    transaction: models::Transaction,
    transactions: &mut Vec<models::Transaction>,
) {
    transactions.push(transaction);
}

pub fn calculate_total_balance(transactions: &Vec<models::Transaction>) -> f64 {
    transactions.iter().map(|t| t.get_amount()).sum()
}

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
