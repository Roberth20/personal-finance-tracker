use std::str::FromStr;

use fin_track::models;
use fin_track::tools;

fn main() {
    let mut transactions: Vec<models::Transaction> = Vec::new();
    // Placeholder with pre-defined value to start process.
    let mut action: models::FlowState = models::FlowState::Unknown;

    println!("Welcome to this personal tracker.");
    while action != models::FlowState::Exit {
        println!("What do you wan to do?");
        println!(
            "1) Add Transaction\t 2) Calculate total balance\t 3) Filter by category\t 4) Exit"
        );
        let result = tools::prompt("# ");
        match result {
            Ok(option) => match models::FlowState::from_str(&option) {
                Ok(state) => {
                    action = state;
                }
                Err(err) => {
                    eprintln!("There was an error parsing your input: '{}'", err);
                    eprintln!("Try again.");
                    action = models::FlowState::Unknown;
                }
            },
            Err(err) => {
                eprintln!("There was an error with your input '{}'", err);
                eprintln!("Try again");
                action = models::FlowState::Unknown
            }
        }

        match action {
            models::FlowState::AddTransaction => println!("Add transaction"),
            models::FlowState::CalculateTotal => println!("Calculate balance"),
            models::FlowState::FilterCategory => println!("Filter category"),
            models::FlowState::Exit => {
                println!("Exit");
                std::process::exit(0);
            }
            models::FlowState::Unknown => println!("Nothing to do"),
        }
    }

    tools::add_transaction(
        models::Transaction::new("Coffee".to_string(), -1.65, models::Category::Food),
        &mut transactions,
    );

    tools::add_transaction(
        models::Transaction::new("Bonus".to_string(), 100.0, models::Category::Salary),
        &mut transactions,
    );

    tools::add_transaction(
        models::Transaction::new("Movistar".to_string(), -10.4, models::Category::Services),
        &mut transactions,
    );

    tools::add_transaction(
        models::Transaction::new("Fish".to_string(), -24.99, models::Category::Food),
        &mut transactions,
    );

    tools::filter_by_category(models::Category::Rent, &transactions);
}
