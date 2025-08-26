use std::collections::HashSet;
use regex::Regex;
use libc::{opendir, readdir, dirent, closedir};
use crate::Result;

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
    let mut result_string = String::new();
    //iterate over hashset
    for file in contents{
        //extract path of wanted file
        if file.contains(&keyword){
            result_string = file;
            
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
            let c_path = std::ffi::CString::new(path).unwrap();
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
                    find_files(filename, store);
                }
                closedir(entries);
            }
         };
    }
}