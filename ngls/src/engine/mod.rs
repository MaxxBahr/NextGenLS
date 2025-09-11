use std::{collections::HashSet, fs};
use regex::Regex;
use libc::{closedir, dirent, opendir, readdir};
use crate::Result;
use crate::results::Filesize;

pub trait FileEnding {
    fn file_ending(&self) -> bool;
}

impl FileEnding for String{
    fn file_ending(&self) -> bool{
        let mut splitter = self.split('/').collect::<Vec<&str>>();
        let file_name = splitter.pop().unwrap();
        let re = Regex::new(r"[a-zA-Z0-9]+\.[a-zA-Z0-9]+").unwrap();
        return re.is_match(file_name);
    }
}

pub fn search_function(path: String, keyword: String)-> Result{
    let contents = collect_files(path);
    //iterate over hashset
    for file_path in contents{
        //open every found file
        if let Ok(result) =  fs::read_to_string(file_path.clone()) {
            if result.contains(&keyword){
                for lines in result.lines(){
                    //return relevant file
                    return Result::new(file_path, 0, lines.to_string(), Filesize::KiB);
                }
            }
        } else{
            continue;
        }
    }
    return Result::default();

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
        let mut corrected_path: String = if path == ".".to_string(){
            std::env::current_dir().unwrap().to_str().unwrap().to_string()
        }else{
            path.clone()
        };
        corrected_path = std::path::Path::new(corrected_path.as_str()).to_str().unwrap().to_string();
        for entry in fs::read_dir(corrected_path).unwrap(){
            let entry = entry.unwrap();
            let path = entry.path();
            let path_string = path.to_str().unwrap().to_string();
            if path.is_dir(){
                find_files(path_string, store);
            }else{
                store.insert(path_string);
            }
        }
    }
}