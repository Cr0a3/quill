use reqwest::*;
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

    pub async fn call(&self, json: ApiCall) -> Result<ApiCall, reqwest::Error> {
        let res = Client::new().post(&self.domain).json(&json).send().await?;
        res.json().await?
    }

    pub async fn get_download_link(&self, name: &String, version: &String) -> String {
        let result = self.call(ApiCall {
            func: "getlink".into(),
            opt1: Some(name.into()),
            opt2: Some(version.into()),
        }).await;

        let json: ApiCall =  match result {
            Ok(j) => j,
            Err(e) => {
                print::error("E", &format!("error while calling the api: {}", e));
                return "none".into();
            },
        };

        let link = json.opt1.expect("error while unwraping download link");
        link

    }
}