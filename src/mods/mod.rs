pub mod get;
pub mod put;
pub mod rm;
pub mod list;
pub mod export;
pub mod import;

extern crate serde_json;

use std::fmt;
use ::rustc_serialize::base64::FromBase64;

// struct that mirrors the json values from consul
// using snake case as #[serde(alias="")] wouldn't work for me
#[allow(non_snake_case)]
#[derive(RustcDecodable, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ValueData {
  CreateIndex: i32,
  ModifyIndex: i32,
  LockIndex: i32,
  Key: String,
  Flags: i32,
  Value: String,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ValueDataOut {
  Key: String,
  Flags: i32,
  Value: String,
}

impl fmt::Display for ValueData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {}",
            self.CreateIndex,
            self.ModifyIndex,
            self.LockIndex,
            self.Key,
            self.Flags,
            self.Value,
        )
    }
}

// take in the json as Vec<ValueData> grab the three fields we want & decode value from base 64
// then reencode to a new struct of Vec<ValueDataOut> so we can remove those three fields
pub fn decode_json(json: Vec<ValueData>) -> Result<String, serde_json::error::Error> {
    let decode_value = json.iter()
        .map(|row| {
            match row.clone() {
                 ValueData {
                    Key: key,
                    Flags: flags,
                    Value: value,
                    ..
                } => ValueDataOut {
                        Key: key,
                        Flags: flags,
                        Value: String::from_utf8(value.from_base64().unwrap()).unwrap(),
                    },
            }
        }).collect::<Vec<_>>();
    serde_json::to_string_pretty(&decode_value)
}
