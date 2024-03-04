/*use std::error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use serde_json::{Value};
use serde::{Serialize, Deserialize};

// CONFIG
static DOMAIN: &str = "127.0.0.1";
static PORT: &str = "7142";
// CONFIG END

#[derive(Serialize, Deserialize)]
pub struct ApiCall<'a> {
    pub api_func: String,
    pub api_func_opt: &'a [u8],
}

impl<'a> ApiCall<'a> {
    pub fn new(_func: String, _func_opt: &'a [u8]) -> Self {
        Self {
            api_func: _func,
            api_func_opt: _func_opt,
        }
    }

    pub fn call(&self) -> Result<Value, std::error::Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", DOMAIN, PORT))?;

        let json = serde_json::to_string(self)?;

        stream.write_all(json.as_bytes())?;
    
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
    
        let v: Value = serde_json::from_str(&buf)?;

        Ok(v)
    }
}
*/