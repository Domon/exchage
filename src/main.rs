extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;
    println!("{:#?}", resp);
    Ok(())
}
