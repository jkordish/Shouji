#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(serde_macros)]
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate docopt;
use docopt::Docopt;
mod mods;

static USAGE: &'static str = "
shouji -- interface with consul

Usage:
    shouji [options] list [<key>]
    shouji [options] get [<key>]
    shouji [options] put [<key>] [<data>]
    shouji [options] export [<file>]
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

";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_server:  String,
    flag_port:    u16,
    arg_key:      Option<String>,
    arg_data:     Option<String>,
    arg_file:     Option<String>,
    cmd_get:      bool,
    cmd_put:      bool,
    cmd_list:     bool,
    cmd_export:   bool,
    cmd_import:   bool,
    flag_help:    bool,
    flag_verbose: bool,
}

fn main() {

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    // we only support get and put right now
    if args.cmd_get {
       mods::get::get(
            &args.flag_server,
            args.flag_port,
            &args.arg_key.unwrap(),
            args.flag_verbose,
        );
    } else if args.cmd_put {
       mods::put::put(
            &args.flag_server,
            args.flag_port,
            &args.arg_key.unwrap(),
            &args.arg_data.unwrap(),
            args.flag_verbose,
        );
    } else if args.cmd_list {
       mods::list::list(
            &args.flag_server,
            args.flag_port,
            &args.arg_key.unwrap(),
            args.flag_verbose,
        );
    }
    else {
        println!("Not sure what to do: {:?}", args);
    }
}
