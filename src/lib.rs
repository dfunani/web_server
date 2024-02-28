use regex::{Captures, Regex};
use requests::{get_http_static, HTTPRequest, HTTPResponse};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    process,
};

const MAX_PORT_NUMBER: u32 = 65535;
pub const IP_REGEX: &str = r"\b(\d+).(\d+).(\d+).(\d+):(\d+)\b";
const HTTP_METHODS_REGEX: &str = r"(GET|POST|PUT|PATCH|DELETE) (/[^\s]*) (HTTP/1.1)";

pub fn get_tcp_connection(ip_address: &str) {
    let listener = TcpListener::bind(&ip_address).unwrap();

    for connection in listener.incoming() {
        let mut stream = connection.unwrap();

        let buffer_reader = BufReader::new(&mut stream);
        let request_type = &buffer_reader.lines().next().unwrap().unwrap()[..];
        let response = process_request(request_type);
        stream.write_all(response.as_bytes()).unwrap();
    }
}

pub fn process_request(request_type: &str) -> String {
    let regex_connection = Regex::new(HTTP_METHODS_REGEX);
    let regex_matched = match regex_connection {
        Ok(regex) => regex,
        Err(_) => return get_response(get_http_static("500"), "error_500"),
    };

    let request_matched = regex_matched
        .captures(request_type)
        .and_then(|captures| captures.get(1))
        .map_or("", |x| x.as_str());

    let mut request_endpoint = regex_matched
        .captures(request_type)
        .and_then(|captures| captures.get(2))
        .map_or("", |x| x.as_str())
        .replace("/", "");

    let mut response: String = String::new();
    let mut allowed = false;

    if request_endpoint == "" {
        request_endpoint = String::from("index");
    }

    println!("{} {}", request_endpoint, request_matched);

    for method in HTTPRequest::allowed_methods() {
        if request_matched == method.as_str() && method.as_str() == "POST" {
            response = get_response(get_http_static("201"), &request_endpoint);
            allowed = true;
        } else if request_matched == method.as_str() && method.as_str() != "POST" {
            response = get_response(get_http_static("200"), &request_endpoint);
            allowed = true;
        }
    }
    if !allowed {
        response = get_response(get_http_static("405"), "error_405");
    }

    response
}

pub fn get_response(mut http_response: HTTPResponse, response_file: &str) -> String {
    let body = fs::read_to_string(format!("{}.html", response_file));

    let content = match body {
        Ok(body_content) => body_content,
        Err(_) => {
            http_response.update(404, "Not Found");
            fs::read_to_string(String::from("error_404.html"))
                .unwrap_or(String::from("No Resource(s) Found."))
        }
    };
    let content_length = content.len();
    let status = format!(
        "HTTP/1.1 {} {}",
        http_response.status_code, http_response.status_message
    );
    let response = format!("{status}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    response
}

pub fn parse_ip(captures: Captures<'_>) -> &str {
    let port_number: u32 = captures.get(5).map_or("", |x| x.as_str()).parse().unwrap();
    if port_number > MAX_PORT_NUMBER {
        println!("Value Error: Not a valid TCP Port.");
        process::exit(1)
    }

    for i in 1..=4 {
        let segment_str: &str = captures.get(i).map_or("", |x| x.as_str());
        if segment_str.len() > 1 && &segment_str[0..1] == "0" {
            println!("Value Error: Not a valid IP Segment.");
            process::exit(1)
        }

        let segment_num: u32 = captures.get(i).map_or("", |x| x.as_str()).parse().unwrap();
        if segment_num > 255 && segment_num <= 0 {
            println!("Value Error: Not a valid IP Segment.");
            process::exit(1)
        }
    }
    captures.get(0).map_or("", |x| x.as_str())
}

pub mod tests {
    use requests::HTTPRequest;

    #[test]
    pub fn test_parse_ip() {
        use crate::{parse_ip, IP_REGEX};
        use regex::Regex;

        let test_data = "127.0.0.1:5000";
        let regex = Regex::new(IP_REGEX).unwrap();
        let response = parse_ip(regex.captures(test_data).unwrap());
        assert_eq!(response, test_data);
    }

    #[test]
    pub fn test_all_http_methods() {
        let methods = HTTPRequest::allowed_methods();
        for method in methods {
            test_process_reequest_error(&method);
            test_process_reequest_success(&method);
        }
    }

    pub fn test_process_reequest_success(method: &HTTPRequest) {
        use crate::process_request;
        let binding = process_request(format!("{:#?} / HTTP/1.1", method).as_str());
        if method.as_str() == "POST" {
            assert_eq!(binding.lines().next().unwrap(), "HTTP/1.1 201 Created")
        } else {
            assert_eq!(binding.lines().next().unwrap(), "HTTP/1.1 200 Ok")
        }
    }

    pub fn test_process_reequest_error(method: &HTTPRequest) {
        use crate::process_request;
        let binding =
            process_request(format!("{:#?} /test_request_endpoint HTTP/1.1", method).as_str());
        assert_eq!(binding.lines().next().unwrap(), "HTTP/1.1 404 Not Found")
    }
}
