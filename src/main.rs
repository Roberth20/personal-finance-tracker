use fin_track::models;
use fin_track::tools::{add_transaction, filter_by_category};

fn main() {
    let mut transactions: Vec<models::Transaction> = Vec::new();

    add_transaction(
        models::Transaction::new("Coffee".to_string(), -1.65, models::Category::Food),
        &mut transactions,
    );

    add_transaction(
        models::Transaction::new("Bonus".to_string(), 100.0, models::Category::Salary),
        &mut transactions,
    );

    add_transaction(
        models::Transaction::new("Movistar".to_string(), -10.4, models::Category::Services),
        &mut transactions,
    );

    add_transaction(
        models::Transaction::new("Fish".to_string(), -24.99, models::Category::Food),
        &mut transactions,
    );

    filter_by_category(models::Category::Rent, &transactions);
}
