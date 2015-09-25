pub mod get;
pub mod put;
pub mod list;

use std::fmt;
use ::rustc_serialize::base64::FromBase64;

// struct that mirrors the json values from consul
// using snake case as #[serde(alias="")] wouldn't work for me
#[allow(non_snake_case)]
#[derive(RustcDecodable, Serialize, Deserialize, Debug, Clone)]
pub struct ValueData {
  pub CreateIndex: i32,
  pub ModifyIndex: i32,
  pub LockIndex: i32,
  pub Key: String,
  pub Flags: i32,
  pub Value: String,
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

// this seems to decode and the base64 values of ValueData::Value but doesn't keep the rest :()
pub fn decode_json (mut json: Vec<ValueData>) -> Result<String, ::serde_json::error::Error> {
    let decode_value: Vec<_> = json.iter_mut()
        .map(|value | {
            (String::from_utf8(value.Value[..].from_base64().unwrap()).unwrap())
        } ).collect();
    ::serde_json::to_string_pretty(&decode_value)
}
