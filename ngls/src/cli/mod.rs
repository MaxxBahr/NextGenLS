use std::{env::{self, Args}, result};
use crate::results::Result;

pub fn choose_argument(raw_result: Result)-> String{
    let argument: Vec<String> = std::env::args().collect();
    let mut result_string = String::new();
    match argument[0].replace("-", "").as_str(){
        "l" | "line" => line(&mut result_string, raw_result),
        "s" | "short" => short(&mut result_string, raw_result),
        "cs" | "case_sensitive" => case_sensitive(&mut result_string, raw_result),
        "pp" | "pretty_print" => pretty_print(&mut result_string, raw_result),
        "k" | "keyword" => keyword(&mut result_string, raw_result),
        "ln" | "linesn" => lines_number(&mut result_string, raw_result),
        _ => String::new()
    }
}

fn line(return_str: &mut String, raw_result: Result)-> String{
    todo!()
}

fn short(return_str: &mut String, raw_result: Result)-> String{
    todo!()
}

fn case_sensitive(return_str: &mut String, raw_result: Result)-> String{
    todo!()
}

fn pretty_print(return_str: &mut String, raw_result: Result)-> String{
    todo!()
}

fn keyword(return_str: &mut String, raw_result: Result)-> String{
    todo!()
}

fn lines_number(return_str: &mut String, raw_result: Result)-> String{
    todo!()
}