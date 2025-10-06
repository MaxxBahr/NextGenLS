use std::ffi::OsString;

use crate::engine::search_function;
use crate::results::Result;

pub fn choose_argument(raw_result: Vec<Result>, argument: String){
    match argument.as_str(){
        "l" | "line" => for res in raw_result{ res.line()},
        "s" | "short" => for res in raw_result{ res.short()},
        "pp" | "pretty_print" => for res in raw_result{ res.pretty_print()},
        "ln" | "linesn" => for res in raw_result{ res.lines_number()},
        _ => ()
    }
}

pub fn get_arguments(){
    let arguments: Vec<String> = std::env::args_os().map(|s| {
        if let Ok(s) = s.into_string() {
            s
        }
        else{
            panic!("No valid arguments given");
        }
    })
    .collect();
    let mut cleaned: Vec<String> = arguments.into_iter()
        .map(|s| {
        if s.starts_with('-'){
            s[1..].to_string()
        }else{
            s
        }
    })
        .collect();
    let path = cleaned.remove(1);
    let keyword = cleaned.remove(1);
    let found = search_function(path, keyword);

    let print_argument: String = cleaned.remove(1);
    choose_argument(found, print_argument.clone());
}