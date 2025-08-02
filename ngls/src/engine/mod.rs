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
    let path_to_string = path;
    //if path is a finite path to file
    //add file to hashset
    //else
    //recall function until file is found
    // make system calls 'readdir' and 'opendir'
}