use std::str::FromStr;

use fin_track::models::{Category, FlowState, Transaction};
use fin_track::tools;
use fin_track::tools::prompt;

fn main() {
    let mut transactions: Vec<Transaction> = Vec::new();
    // Placeholder with pre-defined value to start process.
    let mut action: FlowState = FlowState::Unknown;

    println!("Welcome to this personal tracker.");
    while action != FlowState::Exit {
        println!("What do you wan to do?");
        println!(
            "1) Add Transaction\t 2) Calculate total balance\t 3) Filter by category\t 4) Exit"
        );
        let result = tools::prompt("# ");
        match result {
            Ok(option) => match FlowState::from_str(&option) {
                Ok(state) => {
                    action = state;
                }
                Err(err) => {
                    eprintln!("There was an error parsing your input: '{}'", err);
                    eprintln!("Try again.");
                    action = FlowState::Unknown;
                }
            },
            Err(err) => {
                eprintln!("There was an error with your input '{}'", err);
                eprintln!("Try again");
                action = FlowState::Unknown
            }
        }

        if action == FlowState::Exit {
            println!("Good bye!");
            std::process::exit(0);
        }
        if action == FlowState::Unknown {
            println!("Please, try to select an option again.")
        }

        if action == FlowState::AddTransaction {
            let item_raw = prompt("What is your item to log? # ");
            let Ok(item) = item_raw else {
                eprintln!(
                    "There was an error reading your input: {}",
                    item_raw.unwrap_err()
                );
                continue;
            };
            let input_text = prompt("What is the total? # ");
            let amount = match input_text {
                Ok(amount_raw) => {
                    let Ok(amount) = f64::from_str(&amount_raw) else {
                        eprintln!("Could not convert {amount_raw} to float");
                        continue;
                    };
                    amount.abs()
                }
                Err(m) => {
                    eprintln!("There was an error reading yout input {}", m);
                    continue;
                }
            };

            println!("What category do you assign?");
            println!("Food, Rent, Salary, Services, Expenses Credit");
            let input_text = prompt("# ");
            let category = match input_text {
                Ok(category_raw) => match category_raw.to_lowercase().as_str() {
                    "food" => Category::Food,
                    "rent" => Category::Rent,
                    "salary" => Category::Salary,
                    "services" => Category::Services,
                    "expenses" => Category::Expenses,
                    "credit" => Category::Credit,
                    _ => {
                        eprintln!("{} is not a valid category", category_raw);
                        continue;
                    }
                },
                Err(m) => {
                    eprintln!("There was an error reading yout input {}", m);
                    continue;
                }
            };

            if category != Category::Salary {
                let transaction = Transaction::new(item, -amount, category);
                tools::add_transaction(transaction, &mut transactions);
            } else {
                let transaction = Transaction::new(item, amount, category);
                tools::add_transaction(transaction, &mut transactions);
            }
        }

        if action == FlowState::CalculateTotal {
            println!(
                "Your current total is: {}",
                tools::calculate_total_balance(&transactions)
            );
        }
    }
}
