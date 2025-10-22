mod engine;
mod results;
mod cli;
mod database;
mod indexing;

use crate::cli::get_arguments;
use crate::results::Result;

fn main() {
    get_arguments();
}
