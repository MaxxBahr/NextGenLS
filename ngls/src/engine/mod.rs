use std::{fs, path::Path};
use std::collections::HashSet;

pub fn search_function(path: &Path){
    let contents = fs::read_to_string(path).expect("Could not read file");

}

fn collect_files(path: &Path) -> HashSet<String>{
    let mut result: HashSet<String> = HashSet::new();
    find_files(path, &mut result);
    return result;
}

fn find_files(path: &Path, store: &mut HashSet<String>){
    //if path is a finite path to file
    //add file to hashset
    //else
    //recall function until file is found
}