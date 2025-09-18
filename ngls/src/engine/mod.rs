use std::path::Path;
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
        let file_path_str = file_path.clone();
        let path = Path::new(file_path_str.as_str());
        let file_size = path.metadata().unwrap().len();
        if let Ok(result) =  fs::read_to_string(file_path.clone()) {
            if result.contains(&keyword){
                for lines in result.lines(){
                    //return relevant file
                    if lines.contains(&keyword){
                        return Result::new(file_path, file_size, lines.to_string(), Filesize::KiB);
                    }
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
        //recall function until file is found
        //https://github.com/rust-lang/libc/blob/main/README.md
        // https://docs.rs/libc/latest/libc/fn.opendir.html
        unsafe { 
            let c_path = std::ffi::CString::new(path.clone()).unwrap();
            let entries = opendir(c_path.as_ptr() as *const i8);
            if !entries.is_null() {
                loop {
                    let next_entry = readdir(entries);
                    if next_entry.is_null(){
                        break;
                    }
                    let dir_entry: &dirent = &*next_entry;
                    let c_str: &std::ffi::CStr = std::ffi::CStr::from_ptr(dir_entry.d_name.as_ptr());
                    let filename = c_str.to_string_lossy().into_owned();
                    let full_path = format!("{}/{}", path, filename);
                    find_files(full_path, store);
                }
                closedir(entries);
            }
         };
    }
}