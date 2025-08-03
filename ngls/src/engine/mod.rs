use std::{fs, path::Path};
use std::collections::HashSet;
use regex::Regex;

pub trait file_ending {
    fn file_ending(&self) -> bool;
}

impl file_ending for String{
    fn file_ending(&self) -> bool{
        let mut splitter = self.split('/').collect::<Vec<&str>>();
        let file_name = splitter.pop().unwrap();
        let re = Regex::new(r"[a-zA-Z0-9]+\.[a-zA-Z0-9]+").unwrap();
        return re.is_match(file_name);
    }
}

pub fn search_function(path: String){
    let contents = collect_files(path);
    //iterate over hashset
    //

}

fn collect_files(path: String) -> HashSet<String>{
    let mut result: HashSet<String> = HashSet::new();
    find_files(path, &mut result);
    return result;
}

fn find_files(path: String, store: &mut HashSet<String>){
    //if path is a finite path to file
    if path.file_ending(){
        //add file to hashset
        store.insert(path);
    }
    else{
        //recall function until file is found
        //https://github.com/rust-lang/libc/blob/main/README.md
        // https://docs.rs/libc/latest/libc/fn.opendir.html
        let dir_contents: Vec<String> = Vec::new();
        find_files(path, store);
    }
}