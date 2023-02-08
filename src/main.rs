use std::error::Error;
use serde_json::json;
use serde::{Serialize, Deserialize};

const SOLANA_RPC: &str = "https://api.mainnet-beta.solana.com";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub jsonrpc: String,
    pub result: Result1,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result1 {
    pub context: Context,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub api_version: String,
    pub slot: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub data: Vec<String>,
    pub executable: bool,
    pub lamports: i64,
    pub owner: String,
    pub rent_epoch: i64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let json_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [
            "9oTgUFHWpaxGkemr3qHPqzQmHcaUdB6QFjFZukYT8iXx",
            {
                "encoding": "base64"
            }
        ]

    });


    let response: Quote = client
        .post(SOLANA_RPC)
        .json(&json_body)
        .send()
        .await?
        .json()
        .await?;

    let sol_balance:f64 = response.result.value.lamports as f64 * 0.000000001;
    println!("Sol Balance:\t{} SOL", sol_balance);
    Ok(())
}
