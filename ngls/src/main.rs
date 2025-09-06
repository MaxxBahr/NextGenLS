mod engine;
mod results;
mod cli;

use crate::engine::search_function;
use crate::results::Result;

fn main() {
    let search_result = search_function(".".to_string(), "Hallo".to_string());
    println!("{:?}", search_result);
}
