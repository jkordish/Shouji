extern crate curl;
extern crate rustc_serialize;

use self::curl::http;

pub fn put(server: &str, port: &str, key: &str, data: &str, verbose: bool ) {

    // build url from input values
    let url = format!("http://{}:{}/v1/kv/{}", server, port, key);

    // verbose: print out the connection url string
    if verbose {
        println!("Attempting connection to {} with {}", &url, &data);
    }

    // make connection
    let resp = http::handle()
        .put(url, data)
        .exec()
        .unwrap();

    // expect a 200 code or error with return code
    if resp.get_code() != 200 {
        println!("Unable to handle HTTP response code {}", resp.get_code());
        return;
    }

}
