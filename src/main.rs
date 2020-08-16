use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut key_file = File::open("api-key.json")?;
    let mut contents = String::new();
    key_file.read_to_string(&mut contents)?;

    let key = json::parse(&contents).unwrap();
    let key = key["api_key"].as_str().unwrap();

    let base_url = "http://www.airnowapi.org/aq/observation/zipCode/current/?format=application/json";
    let built_url = format!("{}&zipCode={}&API_KEY={}", base_url, 19123, key);
    let response = reqwest::blocking::get(&built_url).unwrap().text().unwrap();
    println!("{}", response);

    Ok(())
}
