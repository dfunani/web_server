// use std::net::TcpListener;

use std::env::args;

use regex::Regex;
use uuid::Uuid;
use web_server::{get_tcp_connection, parse_ip, exit, IP_REGEX};

fn main() {
    let args: Vec<String> = args().collect();
    let session_id = Uuid::new_v4();
    println!("Web Server Started - Session ID: {}", session_id);

    if args.len() < 2 {
        exit(session_id, 1, "ArgumentParse Error: No IP Address Provided")
    }

    // let ip_address;

    let regex = Regex::new(IP_REGEX).unwrap();
    if !regex.is_match(&args[1]) {
        exit(session_id, 1, "ValueParse Error: Not a valid IP address.")
    }

    if let Some(captures) = regex.captures(&args[1]) {
        let ip_address = parse_ip(captures, session_id);
        let connections = get_tcp_connection(&ip_address);
        match connections{
            Ok(_) => exit(session_id, 0, "TCP Connections Closed"),
            Err(error) => exit(session_id, 1, &error.to_string()) 
        }
    }
    exit(session_id, 0, "TCP Connections Closed")
}


