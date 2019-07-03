use std::str::FromStr;
use base64::decode;
use std::error::Error;
use serde_json::{Map, Value};
use frank_jwt::encode;
use crate::Algorithm;

type KeyValue = Map<String, Value>;

#[derive(Debug)]
pub struct Token {
    header: KeyValue,
    payload: KeyValue,
}

impl FromStr for Token {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(".").collect::<Vec<&str>>();
        let header_json = String::from_utf8(decode(parts[0])?)?;
        let payload_json = String::from_utf8(decode(parts[1])?)?;
        let token = Token {
            header: serde_json::from_str(&header_json)?,
            payload: serde_json::from_str(&payload_json)?,
        };
        Ok(token)
    }
}

impl Token {
    pub fn sign(&self, key: &str, algorithm: Algorithm) -> String {
        let mut header = self.header.clone();
        header.insert("alg".into(), algorithm.to_string().into());
        encode(
            header.into(),
            &key,
            &Value::Object(self.payload.clone()),
            algorithm.unbox()
        ).unwrap()
    }
}
