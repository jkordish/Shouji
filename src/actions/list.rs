extern crate curl;
extern crate serde_json;

use self::curl::http;
use std::str::from_utf8;
use actions::*;

pub fn new(server: &str, port: &str, key: &str, verbose: bool) {

    // build url from input values
    // format the url depending on if a key is set
    let url: String = if key == "" {
        format!("http://{}:{}/v1/kv/?recurse", server, port)
    } else {
        format!("http://{}:{}/v1/kv/{}/?recurse", server, port, key)
    };

    // verbose: print out the connection url string
    if verbose {
        println!("Attempting connection to {}", &url);
    }

    // make connection
    let resp = match http::handle().get(url).exec() {
        Ok(resp) => resp,
        Err(err) => panic!("error executing. {}", err),
    };

    // expect a 200 code or error with return code
    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code())
    }

    // verbose: print out the response code, headers, and body
    if verbose {
        println!("code={}; headers={:?}; body={}",
                 resp.get_code(),
                 resp.get_headers(),
                 from_utf8(resp.get_body()).unwrap());
    }

    // make body from the response body from the server
    let body = match from_utf8(resp.get_body()) {
        Ok(body) => body,
        Err(err) => panic!("{}", err),
    };

    // map json body to our backend Struct
    let json: Vec<ValueData> = match serde_json::from_str(&body[..]) {
        Ok(json) => json,
        Err(err) => panic!("{}", err),
    };

    // pass json to our decode_json function and print result
    match decode_json(json) {
        Ok(json) => println!("{}", json),
        Err(err) => panic!("{}", err),
    }
}
