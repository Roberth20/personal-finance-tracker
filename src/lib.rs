pub mod models;
pub mod tools;
use std::str::FromStr;
use tools::prompt;

/// Helper function to parse input user to some data type
pub fn parse_from_prompt<T: FromStr>() -> Option<T>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    prompt("# ")
        .inspect_err(|e| eprintln!("There was an error reading your input: {e}"))
        .ok()?
        .parse()
        .inspect_err(|e| eprintln!("There was an error parsing your input: {e}"))
        .ok()
}
