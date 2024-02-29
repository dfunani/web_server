use regex::Captures;
use requests::process_request;
use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process,
};
use threads::ThreadPool;
use uuid::Uuid;

const MAX_PORT_NUMBER: u32 = 65535;
pub const IP_REGEX: &str = r"\b(\d+).(\d+).(\d+).(\d+):(\d+)\b";

pub fn get_tcp_connection(ip_address: &str) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(&ip_address)?;

    let thread_pool = ThreadPool::new(4)?;

    for connection in listener.incoming().take(4) {
        let stream = connection?;
        thread_pool.execute(move || {run_thread(stream)});
    }
    Ok(())
}

pub fn run_thread(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&stream);
    let request_type = &buffer_reader.lines().next().unwrap().unwrap()[..];
    let response = process_request(request_type);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn parse_ip(captures: Captures<'_>, session_id: Uuid) -> &str {
    let port_number: u32 = captures.get(5).map_or("", |x| x.as_str()).parse().unwrap();
    if port_number > MAX_PORT_NUMBER {
        exit(session_id, 1, "Value Error: Not a valid TCP Port.");
    }

    for i in 1..=4 {
        let segment_str: &str = captures.get(i).map_or("", |x| x.as_str());
        if segment_str.len() > 1 && &segment_str[0..1] == "0" {
            exit(session_id, 1, "Value Error: Not a valid IP Segment.");
        }

        let segment_num: u32 = captures.get(i).map_or("", |x| x.as_str()).parse().unwrap();
        if segment_num > 255 && segment_num <= 0 {
            exit(session_id, 1, "Value Error: Not a valid IP Segment.");
        }
    }
    captures.get(0).map_or("", |x| x.as_str())
}

pub fn exit(session_id: Uuid, exit_code: i32, message: &str) {
    println!("{message}");
    println!("Web Server Ended - Session ID: {}", session_id);
    process::exit(exit_code);
}

#[cfg(test)]
pub mod tests {
    use uuid::Uuid;

    use crate::{parse_ip, IP_REGEX};


    #[test]
    pub fn test_parse_ip() {
        use regex::Regex;

        let test_data = "127.0.0.1:5000";
        let regex = Regex::new(IP_REGEX).unwrap();
        let response = parse_ip(regex.captures(test_data).unwrap(), Uuid::new_v4());
        assert_eq!(response, test_data);
    }
}
