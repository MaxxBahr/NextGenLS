mod engine;
mod results;
mod cli;

use crate::cli::get_arguments;
use crate::results::Result;

fn main() {
    get_arguments();
}
