use std::path::Path;
use std::process;

pub const DATABASE_FILE: &str = "./keys.db";

pub fn check_dbfile(file_name: &str) {
    if !Path::new(&file_name).exists() {
        println!("Can't find database {}", file_name);
        process::exit(1);
    }
}
