# fin-track — Personal Finance CLI Tracker

A small command-line app for tracking transactions, written to practice core
Rust: structs, enums, ownership/borrowing, and collections.

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
- `src/main.rs` — currently seeds a few hardcoded transactions and calls the functions above as a demo.

## Run it

```sh
cargo run
```

## Test it

```sh
cargo test
```

## Status

The core data model and operations are implemented and unit-tested.
Not yet built: the interactive `std::io` input loop (prompt for
description/amount/category, keep running until "Exit") described in the
original project brief — `main.rs` is still a fixed demo run rather than a
loop reading user input.
