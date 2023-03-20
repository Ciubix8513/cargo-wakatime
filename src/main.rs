use std::{
    env, fs,
    io::{self, Read, Write},
};

fn ask_for_a_key(prompt: Option<&str>) -> Option<String> {
    if let Some(prompt) = prompt {
        println!("{}", prompt);
    }
    println!("Please enter your api key");
    let mut input = String::new();
    let stdin = io::stdin();

    if stdin.read_line(&mut input).is_err() {
        return None;
    }
    //Add potential key validation here
    return Some(input);
}

fn get_key() -> Option<String> {
    let binding = dirs::data_dir()?;
    let data_path = binding.to_str()?;
    let data_path = String::from(data_path) + "cargo-wakatime/";
    println!("{}", data_path);
    fs::create_dir(data_path.clone());

    let data_path = data_path + "key";
    let file = fs::OpenOptions::new().read(true).open(&data_path);
    match file {
        Ok(mut f) => {
            let mut key: String = String::new();
            f.read_to_string(&mut key).unwrap();
            Some(key)
        }
        Err(e) => {
            //This is probably overcomplicating it, but it's fine
            if e.kind().to_string() == "entity not found" {
                //Key file doesn't exist
                //Ask user to enter their key
                let key = ask_for_a_key(Some("Key not found"))?;
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(data_path).unwrap();
                file.write_all(key.as_bytes()).unwrap();
                Some(key)
            } else {
                println!("{}", e);
                None
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let key = get_key().unwrap();
    println!("Key = {}",key);
    println!("args = {:#?}", args);
}
