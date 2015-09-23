pub mod get;
pub mod put;
pub mod list;
use std::fmt;

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

impl fmt::Display for ValueData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {} {} {} {} {}",
            self.CreateIndex,
            self.ModifyIndex,
            self.LockIndex,
            self.Key,
            self.Flags,
            self.Value)
    }
}
