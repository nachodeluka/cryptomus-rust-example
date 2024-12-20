use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use md5;
use reqwest::{self, header};
use serde_json::json;
use std::env;

const API_URL: &str = "https://api.cryptomus.com/v1/payment";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let currency = "USD";
    let order_id = "order12345";
    let amount = 100.0;
    let redirect_url = env::var("REDIRECT_URL").unwrap_or_else(|_| "https://example.com/redirect".to_string());
    let cancel_url = env::var("CANCEL_URL").unwrap_or_else(|_| "https://example.com/cancel".to_string());
    let success_url = env::var("SUCCESS_URL").unwrap_or_else(|_| "https://example.com/success".to_string());

    create_payment(
        amount,
        currency,
        order_id,
        &redirect_url,
        &cancel_url,
        &success_url,
    )
    .await?;

    Ok(())
}

async fn create_payment(
    amount: f64,
    currency: &str,
    order_id: &str,
    redirect_url: &str,
    cancel_url: &str,
    success_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let data = json!({
        "amount": amount,
        "currency": currency,
        "order_id": order_id,
        "redirect_url": redirect_url,
        "cancel_url": cancel_url,
        "success_url": success_url,
    });

    let client = reqwest::Client::new();

    let merchant_id = env::var("CRYPTOMUS_MERCHANT_ID")?;
    let api_key = env::var("CRYPTOMUS_API_KEY")?;

    let json_data = serde_json::to_string(&data)?;
    let escaped_data = json_data.replace('/', "\\/");

    let base64_data = general_purpose::STANDARD.encode(escaped_data);

    let sign = format!("{:x}", md5::compute(format!("{}{}", base64_data, api_key)));

    println!("Sign: {}", sign);

    let response = client
        .post(API_URL)
        .header("merchant", merchant_id)
        .header("sign", sign)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Payment created successfully: {:?}", response.text().await?);
    } else {
        println!("Failed to create payment: {:?}", response.text().await?);
    }

    Ok(())
}
