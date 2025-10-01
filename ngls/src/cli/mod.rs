use crate::results::Result;

pub fn choose_argument(raw_result: Result){
    let argument: Vec<String> = std::env::args().collect();
    match argument[0].replace("-", "").as_str(){
        "l" | "line" => raw_result.line(),
        "s" | "short" => raw_result.short(),
        "pp" | "pretty_print" => raw_result.pretty_print(),
        "ln" | "linesn" => raw_result.lines_number(),
        _ => ()
    }
}