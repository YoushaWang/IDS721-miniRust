use std::error::Error;

#[tokio::main]
async fn fetch() -> Result<(), Box<dyn Error>> {
    let body = reqwest::get("https://finance.yahoo.com/quote/AAPL/")
        .await?
        .text().await?;
    let substring_pos = body.find("data-symbol=\"AAPL\"").unwrap();
    let data = &body[substring_pos..(substring_pos+200)];
    let value_pos = data.find("value=").unwrap();
    let value = &data[(value_pos+7)..(value_pos+12)];
    println!("Apple (AAPL) stock price: {}", value);
    Ok(())
}

fn main(){
    let _res = fetch();
}