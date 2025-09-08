use std::env::{self, Args};

pub fn choose_argument(){
    let argument: Vec<String> = std::env::args().collect();

    match argument.replace("-", "").as_str(){
        "l" | "line" => line(),
        "s" | "short" => short(),
        "cs" | "case_sensitive" => case_sensitive(),
        "pp" | "pretty_print" => pretty_print(),
        "k" | "keyword" => keyword(),
        "ln" | "linesn" => lines_number(),
        _ => ()
    }
}

fn line(){
    todo!()
}

fn short(){
    todo!()
}

fn case_sensitive(){
    todo!()
}

fn pretty_print(){
    todo!()
}

fn keyword(){
    todo!()
}

fn lines_number(){
    todo!()
}