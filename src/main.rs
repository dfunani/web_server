// use std::net::TcpListener;

use std::{env::args, process};

use regex::Regex;
use web_server::{get_tcp_connection, parse_ip, IP_REGEX};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("ArgumentParse Error: No IP Address Provided");
        process::exit(1)
    }

    // let ip_address;

    let regex = Regex::new(IP_REGEX).unwrap();
    if !regex.is_match(&args[1]) {
        println!("Value Error: Not a valid IP address.");
        process::exit(1)
    }


    if let Some(captures) = regex.captures(&args[1]) {
        let ip_address = parse_ip(captures);
        get_tcp_connection(&ip_address);
    }

    process::exit(0);
}

