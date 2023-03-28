#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::stdin;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    info: ApiInfo,
    results: Vec<ApiResult>
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiInfo {
    count: u8,
    pages: u8,
    next: Option<String>,
    prev: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResult {
    id: u8, 
    name: String,
    status: String,
    species: String,
    gender: String,
    image: String

}

fn main() {
    let mut character = String::new();
    println!("Enter Rick and Morty Character name?");
    stdin()
        .read_line(&mut character)
        .unwrap();

    print!("you're searching for: {}", character);

    let result = get_rick_and_morty_character_data(&character).unwrap();
    let data = parse_json(&result).unwrap();

    println!("Results:\n");
    for character_data in data.results {
        println!("{:?} -> {:?}", character_data.name, character_data.image);
    }
}


#[tokio::main]
async fn get_rick_and_morty_character_data(character_name: &String) -> Result<String, Box<dyn Error>>{
    let mut base_url = String::from("https://rickandmortyapi.com/api/character/?name=");
    base_url.push_str(character_name);

    println!("hitting on: {}", base_url);

    let res = reqwest::get(base_url )
        .await?
        .text()
        .await?;

    Ok(res)
}

fn parse_json(json_response: &String) -> serde_json::Result<ApiResponse> {
    let data: ApiResponse = serde_json::from_str(json_response)?;
    Ok(data)
}