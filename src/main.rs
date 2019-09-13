extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY").expect("Environment variable API_KEY is not set");
    let given_amount = std::env::args().nth(1).expect("Base amount is not given");

    let q = "USD_AUD";
    let url = format!(
        "https://free.currconv.com/api/v7/convert?q={q}&compact=ultra&apiKey={api_key}",
        q = q,
        api_key = api_key
    );

    let response: HashMap<String, f64> = reqwest::get(&url)?.json()?;
    let rate = response[q];

    let base_amount: f64 = given_amount.parse().unwrap();
    let target_amount = base_amount * rate;

    println!("Given Amount: {:#?}", given_amount);
    println!("Base Amount: {:#?}", base_amount);
    println!("Rate: {:#?}", rate);
    println!("Target Amount: {:#?}", target_amount);

    Ok(())
}
