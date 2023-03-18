use std::{env, fs, io::Read};

fn ask_for_a_key() -> Option<String> {
    todo!();
}

fn get_key() -> Option<String> {
    let binding = dirs::data_dir()?;
    let data_path = binding.to_str()?;
    let data_path = String::from(data_path) + "cargo-wakatime/";
    println!("{}", data_path);
    fs::create_dir(data_path.clone()).unwrap();
    let data_path = data_path + "key";
    let file = fs::OpenOptions::new().read(true).open(data_path);
    match file {
        Ok(mut f) => {
            let mut key: String = String::new();
            f.read_to_string(&mut key).unwrap();
            Some(key)
        }
        Err(e) => {
            if e.kind().to_string() == "entity not found" {
                //Key file doesn't exist
                //Ask user to enter their key
                ask_for_a_key()
            } else {
                println!("{}", e);
                None
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    get_key();
    println!("args = {:#?}", args);
}
