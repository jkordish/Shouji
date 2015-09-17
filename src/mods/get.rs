extern crate curl;
extern crate rustc_serialize;

use ::rustc_serialize::base64::FromBase64;
use std;
use self::curl::http;
use mods::ValueData;

pub fn get(server: &str, port: u16, key: &str, verbose: bool ) {

    // build url from input values
    let url = format!("http://{}:{:?}/v1/kv/{}", server, port, key);

    // verbose: print out the connection url string
    if verbose {
        println!("Attempting: {}", &url);
    }

    // make connection
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap();

    // expect a 200 code or error with return code
    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
        return;
    }

    // store the body of the return message
    let body: &str = std::str::from_utf8(resp.get_body()).unwrap();

    // verbose: print out the response code, headers, and body
    if verbose {
        println!("code={}; headers={:?}; body={:?}",
            resp.get_code(), resp.get_headers(), &body);
    }

    // create a vector containg the body
    let json = vec![&body[..]];

    // if verbose: print out the data from the json vec
    if verbose {
        println!("json: {}", json[0]);
    }

    // not sure why but doing inter over json works
    for x in json {
        // decode json to the ValueData struct
        let deserialized: Vec<ValueData> = ::serde_json::from_str(&x).unwrap();
        // convert ValueData.Value from base64 utf8
        let value = deserialized[0].Value[..].to_owned().from_base64().unwrap();
        // Print the string of value
        println!("{}", String::from_utf8(value).unwrap());
    };

}
