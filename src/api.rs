use std::{fs::File, io::{self, Read}};

use reqwest:: Client;
use crate::print;

pub struct Api {
    pub domain: String,  
}

impl Api {
    pub fn new(domain: &str) -> Self {
        Self {
            domain: domain.into()
        }
    }

    pub async fn download(&self, name: &String, version: &String, path: &String) -> Result<bool, reqwest::Error> {
        let res =  Client::new().post(&format!("{}?func=download", self.domain))
            .form(&[
                ("name", name),
                ("version", version)
            ]).send().await?;

        let buf =  res.text().await?;

        if buf.chars().next() == 'e'.into() {
            print::error("E", &format!("{}", buf));
            return Ok(false);
        }

        let mut out = File::create(path).expect("failed to create file");
        io::copy(&mut buf.as_bytes(), &mut out).expect("failed to copy downloded content");

        Ok(true)
    }

    pub async fn upload(&self, path: &String) -> Result<bool, reqwest::Error> {

        let mut file = File::open(path).expect("error while opening file");
        let mut buf: String = String::new();

        match file.read_to_string(&mut buf) {
            Ok(_) => {},
            Err(e) => {
                print::error("E", &format!("error while reading target file: {}", e));
                return Ok(false);
            },
        };

        let res =  Client::new().post(&format!("{}?func=upload", self.domain))
            .form(&[
                ("vZip", buf)
            ]).send().await?;

        let buf =  res.text().await?;

        if buf.chars().next() == 'e'.into() {
            print::error("E", &format!("{}", buf));
            return Ok(false);
        }

        Ok(true)
    }
}