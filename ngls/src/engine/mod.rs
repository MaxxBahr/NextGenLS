use std::{fs, path::Path};
use std::collections::HashSet;
use regex::Regex;

pub fn search_function(path: &Path){
    let contents = collect_files(path);
    //iterate over hashset
    //

}

fn collect_files(path: &Path) -> HashSet<String>{
    let mut result: HashSet<String> = HashSet::new();
    find_files(path, &mut result);
    return result;
}

fn find_files(path: &Path, store: &mut HashSet<String>){
    let path_to_string = path.to_str().unwrap();
    //if path is a finite path to file
    //add file to hashset
    match path_to_string.ends_with('.')
    //else
    //recall function until file is found
}

fn file_ending(path: String) -> bool{

}