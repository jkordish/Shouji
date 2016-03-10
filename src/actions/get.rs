extern crate curl;
extern crate rustc_serialize;

use rustc_serialize::base64::FromBase64;
use std::str::from_utf8;
use self::curl::http;
use actions::ValueData;

pub fn new(server: &str, port: &str, key: &str, verbose: bool) {

    // build url from input values
    let url = format!("http://{}:{}/v1/kv/{}", server, port, key);

    // verbose: print out the connection url string
    if verbose {
        println!("Attempting: {}", &url);
    }

    // make connection
    let resp = match http::handle().get(url).exec() {
        Ok(resp) => resp,
        Err(err) => panic!("error executing. {}", err),
    };

    // expect a 200 code or error with return code
    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
        return;
    }

    // verbose: print out the response code, headers, and body
    if verbose {
        println!("code={}; headers={:?}; body={}",
                 resp.get_code(),
                 resp.get_headers(),
                 from_utf8(resp.get_body()).unwrap());
    }

    // create a vector containg the body
    let body = vec![resp.get_body()];

    // not sure why but doing inter over json works
    for row in body {
        // if verbose: print out the data from the json vec
        if verbose {
            match from_utf8(row) {
                Ok(row) => println!("JSON body. {}", row),
                Err(err) => panic!("unable to convert body. {}", err),
            }
        }
        // decode json to the ValueData struct
        let deserialized: Vec<ValueData> = match ::serde_json::from_slice(row) {
            Ok(deserialized) => deserialized,
            Err(err) => panic!("{}", err),
        };

        // convert ValueData.Value from base64 utf8
        let value = match deserialized[0].Value[..].to_owned().from_base64() {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        };

        // Print the string of value
        match String::from_utf8(value) {
            Ok(x) => println!("{}", x),
            Err(err) => panic!("{}", err),
        };
    }
}
