use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut key_file = File::open("api-key.json")?;
    let mut contents = String::new();
    key_file.read_to_string(&mut contents)?;

    let key = json::parse(&contents).unwrap();
    let key = key["api_key"].as_str().unwrap();

    let client = airnow_aqi::Airnow::new(key.to_string());
    let obs = client.get_current_observations(19123, Some(25)).unwrap();
    for element in obs {
        println!("{:?}", element);
    }

    Ok(())
}
