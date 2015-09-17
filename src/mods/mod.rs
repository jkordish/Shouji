pub mod get;

// struct that mirrors the json values from consul
// using snake case as #[serde(alias="")] wouldn't work for me
#[allow(non_snake_case)]
#[derive(RustcDecodable, Serialize, Deserialize, Debug)]
pub struct ValueData {
  pub CreateIndex: i32,
  pub ModifyIndex: i32,
  pub LockIndex: i32,
  pub Key: String,
  pub Flags: i32,
  pub Value: String,
}
