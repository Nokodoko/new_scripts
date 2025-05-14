use reqwest::{self, header};
//use serde::{serielize, deserielize};
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, 
        header::HeaderValue::from_static(""));

    let mfa_list = reqwest::Client::buildr()
        .default_headers(headers)
        .build()?;

    let client = mfa_list
        //.get("https://lastpass.com/enterpriseapi.php")
        .get("https://api.github.com/user/nokodoko")
        .send()
        .await?
        .json()  //convert to text if failes.
        .await?;
        //write to a function -> stringify function - text below is a place holder for function
        //name


    println!("Result: {:?}", client); 
    Ok(())
}
