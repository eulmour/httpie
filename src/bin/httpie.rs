use std::{collections::HashMap, sync::Arc, time::Instant};
use httpie::srv::Server;
use httpie::srv::Request;
use httpie::srv::Response;

mod route;

const DEFAULT_ADDRESS: &'static str = "127.0.0.1:8080";

const CLI_HELP_MSG: &'static str = "httpie 0.1.1\n\
    \n\
    USAGE:\n\
        main [OPTIONS]\n\
    \n\
    FLAGS:\n\
        -h, --help       Prints help information\n\
        -V, --version    Prints version information\n\
    \n\
    OPTIONS:\n\
        -a, --address <ADDRESS>    Sets address:port\n\
        -d, --dir <DIRECTORY>      Sets public directory\n\
    \n\
    httpie reads HTTPIE_ADDRESS environment variable";

fn main() {

    // command line parsing
    let args = {
        let mut result: (
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>) = Default::default();

        for (index, arg) in std::env::args().enumerate() {
            if arg == "-a" || arg == "--address" {
                if result.0.is_some() { panic!("Invalid arguments."); }
                result.0 = std::env::args().nth(index + 1)
            }
            if arg == "-d" || arg == "--dir" {
                if result.1.is_some() { panic!("Invalid arguments."); }
                result.1 = std::env::args().nth(index + 1)
            }
            if arg == "-h" || arg == "--help" {
                if result.2.is_some() { panic!("Invalid arguments."); }
                result.2 = Some(arg.clone())
            }
            if arg == "-V" || arg == "--version" {
                if result.3.is_some() { panic!("Invalid arguments."); }
                result.3 = Some(arg)
            }
        }

        result
    };

    // information
    if args.2.is_some() {
        println!("{}", CLI_HELP_MSG);
        return;
    }

    if args.3.is_some() {
        println!("{}", "Version 0.1.1");
        return;
    }

    // getting address binding
    let address = match args.0 {
        Some(val) => val,
        None => match std::env::var_os("HTTPIE_ADDRESS") {

            Some(addr) => addr.into_string().expect("Bad OS variable"), //TODO to test

            None => {
                println!("No configuration specified. Using defaults: {}", DEFAULT_ADDRESS);
                DEFAULT_ADDRESS.to_owned()
            }
        }
    };

    let start_time = Instant::now();

    Server::new()
        .address(&address)
        .public(args.1.as_ref().unwrap_or(&"www".to_owned()))
        .max_connections(4)
        .routes(Arc::new(HashMap::from([
            ("/hello", Arc::new(route::hello_world) as Arc<dyn Fn(Request) -> Response + Send + Sync>),
            ("/cwd", Arc::new(route::route_cwd))
        ])))
        .run();

    println!("Time elapsed: {} s. Shutting down...", start_time.elapsed().as_secs());
}
