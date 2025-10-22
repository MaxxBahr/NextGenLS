use crate::database::db_structs::DbError;

mod db_structs;

trait Read<T>{
    fn read<F>(f: F) -> Vec<T>;
    fn get_by_id(id: u64) -> Vec<T>;
    fn find(query: String) -> Vec<T>;
    fn all() -> Vec<T>;
    fn exists(id: u64)-> bool;
    fn count() -> usize;
}

trait Write<T>{
    fn write(t: T) -> Result<String, DbError>;
    fn delete(id: u64) -> Result<String, DbError>;
    fn update(id: u64) -> Result<(), DbError>;
    fn bulk_insert(t: Vec<T>) -> Result<(), DbError>;
}

trait Transactional {

}

trait Database<T>: Read<T> + Write<T> + Transactional {

}