use fin_track::models::{Category, FlowState, Transaction};
use fin_track::parse_from_prompt;
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

        if let Some(state) = parse_from_prompt::<FlowState>() {
            action = state;
        } else {
            action = FlowState::Unknown;
        }

        if action == FlowState::Exit {
            println!("Good bye!");
            std::process::exit(0);
        }
        if action == FlowState::Unknown {
            println!("Please, try to select an option again.");
            continue;
        }

        if action == FlowState::AddTransaction {
            println!("What is your item to log?");
            let item_raw = prompt("# ");
            let Ok(item) = item_raw else {
                eprintln!(
                    "There was an error reading your input: {}",
                    item_raw.unwrap_err()
                );
                continue;
            };

            println!("What category do you assign?");
            println!("Food, Rent, Salary, Services, Expenses Credit");
            let category = if let Some(category) = parse_from_prompt::<Category>() {
                category
            } else {
                continue;
            };

            println!("What is the total?");
            let amount = if let Some(amount) = parse_from_prompt::<f64>() {
                if category != Category::Salary {
                    -amount.abs()
                } else {
                    amount.abs()
                }
            } else {
                continue;
            };

            tools::add_transaction(Transaction::new(item, amount, category), &mut transactions);
        }

        if action == FlowState::CalculateTotal {
            println!(
                "Your current total is: {}",
                tools::calculate_total_balance(&transactions)
            );
        }

        if action == FlowState::FilterCategory {
            println!("What category do you want to explore?");
            println!("Food, Rent, Salary, Services, Expenses Credit");
            let category = if let Some(category) = parse_from_prompt::<Category>() {
                category
            } else {
                continue;
            };

            tools::filter_by_category(category, &transactions);
        }
    }
}
