#![feature(custom_derive, plugin, custom_attribute, type_macros)]
#![plugin(serde_macros, docopt_macros)]
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
mod actions;

docopt!(Args derive, "
shouji -- interface with consul

Usage:
    shouji [options] list [<key>]
    shouji [options] get [<key>]
    shouji [options] put [<key>] [<data>]
    shouji [options] rm [<key>]
    shouji [options] export [<key>] <file>
    shouji [options] import [<file>]
    shouji (-h)

Options:
    -h, --help              show this screen
    -v, --verbose           be verbose
    -s --server <host>      consul server to connect [default: localhost]
    -p --port <port>        consul server port to connect [default: 8500]

Action:
    list    list recursively from location
    get     get raw value from location
    put     put data into location
    export  export to json
    import  import from json

");

fn main() {

    // Decode docopts
    let args: Args = Args::docopt()
                         .decode()
                         .unwrap_or_else(|e| e.exit());


    if args.cmd_get {
        // Error conditions
        if &args.arg_key == "" {
            panic!("Please supply a key to get.");
        }
        actions::get::get(&args.flag_server,
                          &args.flag_port,
                          &args.arg_key,
                          args.flag_verbose);
    } else if args.cmd_put {
        // Error conditions
        if &args.arg_key == "" {
            panic!("Please supply a key to get.");
        }
        if &args.arg_data == "" {
            panic!("Please supply data to put.");
        }
        actions::put::put(&args.flag_server,
                          &args.flag_port,
                          &args.arg_key,
                          &args.arg_data,
                          args.flag_verbose);
    } else if args.cmd_rm {
        // Error conditions
        if &args.arg_key == "" {
            panic!("Please supply a key to rm.");
        }
        actions::remove::rm(&args.flag_server,
                            &args.flag_port,
                            &args.arg_key,
                            args.flag_verbose);
    } else if args.cmd_list {
        // Error conditions
        // if args.arg_key == None { let Some = String::new(); };
        actions::list::list(&args.flag_server,
                            &args.flag_port,
                            &args.arg_key,
                            args.flag_verbose);
    } else if args.cmd_export {
        // Error conditions
        if &args.arg_file == "" {
            panic!("Please supply a file to export to.");
        }
        actions::export::export(&args.flag_server,
                                &args.flag_port,
                                &args.arg_key,
                                &args.arg_file,
                                args.flag_verbose);
    } else if args.cmd_import {
        // Error conditions
        if &args.arg_file == "" {
            panic!("Please supply a file to import.");
        }
        actions::import::import(&args.flag_server,
                                &args.flag_port,
                                &args.arg_file,
                                args.flag_verbose);
    } else {
        println!("Not sure what to do");
    }
}
