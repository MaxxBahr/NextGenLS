use std::hash::{BuildHasherDefault, DefaultHasher};
use std::path::Path;
use std::sync::Mutex;
use std::{collections::HashSet, fs, io};
use regex::Regex;
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

// Main entry point of the engine
pub fn search_function(path: String, keyword: String)-> Vec<Result>{
    let contents = collect_files(path);
    let mut search_results: Vec<Result> = Vec::new();
    //iterate over hashset
    for file_path in contents{
        //open every found file
        let file_path_str = file_path.clone();
        let path = Path::new(file_path_str.as_str());
        let file_size = path.metadata().unwrap().len();
        if let Ok(result) =  fs::read_to_string(file_path.clone()) {
            if result.contains(&keyword){
                for lines in result.lines(){
                    //return relevant file
                    if lines.contains(&keyword){
                        search_results.push(Result::new(file_path.clone(), file_size, lines.to_string(), Filesize::KiB)) ;
                    }
                }
            }
        } else{
            continue;
        }
    }
    return search_results;

}

unsafe fn collect_files(path: String) -> Mutex<HashSet<String, BuildHasherDefault<DefaultHasher>>>{
    // Use different caller for HashSet since seed is not static this way
    // https://doc.rust-lang.org/std/collections/struct.HashSet.html
    static mut SET: Mutex<HashSet<String, BuildHasherDefault<DefaultHasher>>> = Mutex::new(HashSet::with_hasher(BuildHasherDefault::new()));
    let _ = find_files(path, &mut SET);
    return SET;
}

fn find_files(path: String, store: &mut Mutex<HashSet<String, BuildHasherDefault<DefaultHasher>>>)-> io::Result<()>{
    let mut locked_store = store.lock().unwrap();
    //if path is a finite path to file
    if path.file_ending(){
        //add file to hashset
        locked_store.insert(path);
        Ok(())
    }
    else{
        let mut corrected_path: String = if path == ".".to_string(){
            std::env::current_dir()?.to_str().unwrap().to_string()
        }else{
            path.clone()
        };
        corrected_path = std::path::Path::new(corrected_path.as_str()).to_str().unwrap().to_string();
        for entry in fs::read_dir(corrected_path)?{
            let entry = entry?;
            let path = entry.path();
            let path_string = path.to_str().unwrap().to_string();
            if path.is_dir(){
                let handle = std::thread::spawn(move ||{
                    let _ = find_files(path_string, store);
                });
                handle.join().unwrap();
            }else{
                locked_store.insert(path_string);
            }
        }
        Ok(())
    }
}