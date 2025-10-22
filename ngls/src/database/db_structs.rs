use std::collections::HashMap;
use chrono::prelude::*;

pub enum DbError{
    ConnErr(String),
    ValErr(String),
    AuthErr(String),
    AccErr(String)
}
enum MimeType{
    Html,
    Css,
    JavaScript,
    Json,
    Xml,
    PlainText,
    Csv,
    Binary,
    Pdf,
    Png,
    Jpeg,
    Gif,
    Svg,
    WebP,
    Mp3,
    Mp4,
    Ogg,
    WebM,
    Woff,
    Woff2,
    Ttf,
    Eot,
    Application,
    Unknown,
}

struct DbEntry{
    primary_key: i32,
    filesize: f32,
    filename: String,
    path: String,
    is_dir: bool,
    parent_id: Option<i32>,
    created_at: chrono::DateTime<Local>,
    depth_level: i16,
    modified_at: chrono::DateTime<Local>,
    extension: String,
    mime_type: Option<MimeType>,
}

pub struct DbResult{
    rows: HashMap<String, DbEntry>
}

pub struct DbConnection {
    url: String,
    port: i32,
}

impl DbConnection {
    pub fn new(url: String, port: i32) -> DbConnection {
        DbConnection { url, port }
    }

    pub fn create_connection(&self) -> Result<String, String>{
        let conn_string = format!("{}:{}", self.url, self.port);
        Err("Connection not possible".to_string())
    }
}