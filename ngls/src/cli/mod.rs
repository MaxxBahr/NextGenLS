use crate::engine::search_function;
use crate::results::Result;

pub fn choose_argument(raw_result: Result, argument: String){
    match argument.as_str(){
        "l" | "line" => raw_result.line(),
        "s" | "short" => raw_result.short(),
        "pp" | "pretty_print" => raw_result.pretty_print(),
        "ln" | "linesn" => raw_result.lines_number(),
        _ => ()
    }
}

pub fn get_arguments(){
    let arguments: Vec<String> = std::env::args().collect();
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
    let keyword = cleaned.remove(2);
    let found = search_function(path, keyword);

    let print_argument: String = cleaned.remove(3);
    choose_argument(found, print_argument);
}