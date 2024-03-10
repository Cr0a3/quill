use serde_json::{json, Value};
use reqwest::{self, Client};
use serde::{Serialize, Deserialize};
use crate::print;

#[derive(Debug, Serialize, Deserialize)]
struct ApiCall {
    func: String,
    opt1: Option<String>,
    opt2: Option<String>,
}

pub struct Api {
    pub domain: String,  
}

impl Api {
    pub fn new(domain: &str) -> Self {
        Self {
            domain: domain.into()
        }
    }

    pub async fn call(&self, json: Value) -> Result<Value, reqwest::Error> {
        let client = Client::new();
        let res = client.post(&self.domain).json(&json).send().await?;
        let answer: Value = res.json().await?;

        Ok(answer)
    }

    pub async fn get_download_link(&self, name: &String, version: &String) -> String {
        let result = self.call(json!({
            "func": "download",
            "name": name,
            "version": version,
        })).await;

        let json: Value =  match result {
            Ok(j) => j,
            Err(e) => {
                print::error("E", &format!("error while calling the api: {}", e));
                return "none".into();
            },
        };

        let link = json["link"].to_string();

        if link == "error" {
            return json["error"].to_string()
        }
        
        link

    }
}