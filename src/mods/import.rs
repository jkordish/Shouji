extern crate curl;
extern crate serde_json;

use self::curl::http;
use std::str::from_utf8;
use std::io::prelude::*;
use std::fs::File;
use mods::*;

pub fn import(server: &str, port: &str, file: &str, verbose: bool) {

    // determine if we can open the file for reading
    // panic if we can't
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(..)  => panic!("unable to open file"),
    };

    // create data as a mutable string
    let mut data = String::new();
    // read data from the file and put it in data
    &file.read_to_string(&mut data).unwrap();

    // convert the data to our struct so we can match things up
    let json: Vec<ValueDataOut> = serde_json::from_str(&data[..]).unwrap();

    // iterate over the struct so we can grab values
    for item in json.iter() {
        // build url from input values
        let url = format!("http://{}:{}/v1/kv/{}", server, port, &item.Key);

        // verbose: print out the connection url string
        if verbose {
            println!("Attempting connection to {} with {}", &url, &item.Value[..]);
        }

        // make connection
        let resp = http::handle()
            .put(url, &item.Value[..])
            .exec()
            .unwrap();

        // expect a 200 code or error with return code
        if resp.get_code() != 200 {
            println!("Unable to handle HTTP response code {}", resp.get_code())
        }

        // verbose: print out the response code, headers, and body
        if verbose {
            println!("code={}; headers={:?}; body={}",
                resp.get_code(), resp.get_headers(), from_utf8(resp.get_body()).unwrap());
        }
    };
}
