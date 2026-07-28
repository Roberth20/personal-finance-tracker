# fin-track — Personal Finance CLI Tracker

A small command-line app for tracking transactions, written to practice core
Rust: structs, enums, ownership/borrowing, and collections.

## Original brief

> Project Idea: "The Personal Finance CLI Tracker"
> Create a command-line application that allows a user to manage a small list of transactions.
>
> Requirements to showcase your knowledge:
> - Data Structures: Define a struct called Transaction that holds a description (String), an amount (f64), and a category (use an enum for categories like Food, Rent, Salary, etc.).
> - Ownership & Collections: Use a Vec\<Transaction\> to store your list of transactions. You must demonstrate how to add new transactions to this vector.
> - Functions: Create separate functions for:
>   - add_transaction(...)
>   - calculate_total_balance(...) (This should borrow the vector, not take ownership).
>   - filter_by_category(...) (This should return a new list or print transactions matching a specific enum variant).
> - Control Flow: Use a match statement when handling the category enum and a loop or while loop to keep the program running until the user chooses to "Exit".
> - Input Handling: Use std::io to take input from the user (e.g., "Enter amount: "). Note: Remember to handle parsing strings to numbers safely.
>
> Why this tests your basics:
> - Ownership: You will have to think about whether functions should borrow the Vec or take ownership of it.
> - Types & Enums: You'll practice mapping user input (strings) to strongly-typed Enum variants.
> - Mutability: You will need to make your Vec mutable to add items, while keeping other parts of your logic immutable.

## What it does

Manage a list of transactions, each with a description, an amount, and a
category:

```rust
pub enum Category {
    Food,
    Rent,
    Salary,
    Services,
    Expenses,
    Credit,
}

pub struct Transaction {
    pub description: String,
    pub amount: f64,
    pub category: Category,
}
```

## Project layout

- `src/models.rs` — `Transaction` and `Category` types.
- `src/tools.rs` — operations on a `Vec<Transaction>`:
  - `add_transaction(transaction, &mut transactions)` — pushes a transaction (takes ownership of the transaction, borrows the vec mutably).
  - `calculate_total_balance(&transactions) -> f64` — sums amounts, borrows the vec immutably.
  - `filter_by_category(category, &transactions)` — prints transactions matching a category, using `match`/`==` on the enum.
- `src/main.rs` — interactive loop: prompts for an action (add / total / filter / exit), reads description/category/amount via `std::io`, keeps running until "Exit".
- `src/lib.rs` — `parse_from_prompt<T: FromStr>()`, a generic helper shared by every "read a line, parse it into a type" spot (`Category`, `FlowState`, `f64`).

## Run it

```sh
cargo run
```

## Test it

```sh
cargo test
```

## Status

All requirements from the original brief are implemented: struct + enum data
model, `Vec<Transaction>` with add/borrow/filter functions, `match`-based
enum parsing, an input loop that runs until "Exit", and safe (non-panicking)
`std::io` parsing. `cargo test` and `cargo clippy` are both clean.

## Known gaps & improvement opportunities

Ranked by value for the least effort:

1. **`filter_by_category` prints instead of returning a list** (`tools.rs`) —
   can't be unit-tested without capturing stdout. Returning
   `Vec<&Transaction>` costs one line and makes it testable.
2. **No tests for `Category::from_str` / `FlowState::from_str`** — the actual
   "map user strings to enum variants" logic the exercise is meant to
   exercise has zero coverage today.
3. **`FlowState` is branched with `if`/`if`/`if` instead of `match`**
   (`main.rs`) — works, but loses the compiler's exhaustiveness check if a
   new variant is ever added.
4. **Sign-flip business rule lives in `main.rs`** (auto-negate non-`Salary`
   amounts) — untestable at that layer and not something the brief asked
   for; move it into `tools.rs` as a plain function, or drop it.
5. **`std::process::exit(0)` on Exit** (`main.rs`) is redundant — the `while`
   loop already ends there; falling off `main` does the same thing.

None of these block the exercise's goals (ownership, enums, borrowing,
`match`, safe input parsing are all demonstrated) — they're the next round of
polish, not missing requirements.
