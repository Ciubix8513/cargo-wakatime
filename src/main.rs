use std::{env, path::PathBuf,fs};
use dirs;

fn get_key() -> Option<String>{
    let mut data_path = dirs::data_dir()?.to_str()?;
    let data_path = data_path + "cargo-wakatime";
    Some(String::from(""))
}


fn main() {
    let args: Vec<String> = env::args().collect();
    get_key();
    println!("args = {:#?}",args);
}


