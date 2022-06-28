mod country;

use reqwest;
use country_emoji::flag;   
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Hello, world 🌏!");

    let client = reqwest::Client::new();
    // let res = client.get("https://restcountries.com/v3.1/name/japan")
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // let parsed_res: Value = serde_json::from_str(&res).unwrap();
    // let country_code = parsed_res[0]["cca2"].as_str().unwrap();
    // let capital = parsed_res[0]["capital"][0].as_str().unwrap();
    // let flag = flag(country_code).unwrap();

    // println!(
    //     "Country code: {} \nCapital city: {} \nCountry flag: {}", country_code, capital, flag
    // );

    let url = "https://restcountries.com/v3.1/name/japan";
    let res = client
        .get(url)
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {
            let x = res.text().await.unwrap();
            let parsed_res: Value = serde_json::from_str(&x).unwrap();
            let country_code = parsed_res[0]["cca2"].as_str().unwrap();
            let capital = parsed_res[0]["capital"][0].as_str().unwrap();
            let flag = flag(country_code).unwrap();
            println!(
                "Country code: {} \nCapital city: {} \nCountry flag: {}", country_code, capital, flag
            );
        },
        _ => {
            panic!("Request failed with status code: {}", res.status());
        },
    }
    
    // println!("{:?}", res);
    Ok(())
    
}
