use std::{collections::HashMap, fs};

use regex::Regex;

const HTTP_METHODS_REGEX: &str = r"(GET|POST|PUT|PATCH|DELETE) (/[^\s]*) (HTTP/1.1)";
#[derive(Clone)]
pub struct HTTPResponse {
    pub status_code: u32,
    pub status_message: String,
}

impl HTTPResponse {
    pub fn new(status_code: u32, status_message: &str) -> Self {
        HTTPResponse {
            status_code: status_code,
            status_message: status_message.to_string(),
        }
    }

    pub fn update(&mut self, status_code: u32, status_message: &str) {
        self.status_code = status_code;
        self.status_message = status_message.to_string();
    }
}

pub fn get_http_static(key: &str) -> HTTPResponse {
    let mut http_map: HashMap<String, HTTPResponse> = HashMap::new();
    http_map.insert("200".to_string(), HTTPResponse::new(200, "Ok"));
    http_map.insert("201".to_string(), HTTPResponse::new(201, "Created"));
    http_map.insert(
        "500".to_string(),
        HTTPResponse::new(500, "Internal Server Error"),
    );
    http_map.insert(
        "405".to_string(),
        HTTPResponse::new(405, "Method Not Allowed"),
    );
    match http_map.get(key) {
        Some(value) => value.clone(),
        None => HTTPResponse::new(404, "Not Found"),
    }
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

#[derive(Debug)]
pub enum HTTPRequest {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HTTPRequest {
    pub fn as_str(&self) -> &str {
        match self {
            HTTPRequest::GET => "GET",
            HTTPRequest::POST => "POST",
            HTTPRequest::PUT => "PUT",
            HTTPRequest::PATCH => "PATCH",
            HTTPRequest::DELETE => "DELETE",
        }
    }

    pub fn allowed_methods() -> Vec<HTTPRequest> {
        vec![
            HTTPRequest::GET,
            HTTPRequest::POST,
            HTTPRequest::PUT,
            HTTPRequest::PATCH,
            HTTPRequest::DELETE,
        ]
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

#[cfg(test)]
mod tests {
    use crate::{process_request, HTTPRequest};

    #[test]
    pub fn test_all_http_methods() {
        let methods = HTTPRequest::allowed_methods();
        for method in methods {
            test_process_reequest_error(&method);
            test_process_reequest_success(&method);
        }
    }

    pub fn test_process_reequest_success(method: &HTTPRequest) {
        let binding = process_request(format!("{:#?} / HTTP/1.1", method).as_str());
        if method.as_str() == "POST" {
            assert_eq!(binding.lines().next().unwrap(), "HTTP/1.1 201 Created")
        } else {
            assert_eq!(binding.lines().next().unwrap(), "HTTP/1.1 200 Ok")
        }
    }

    pub fn test_process_reequest_error(method: &HTTPRequest) {
        let binding =
            process_request(format!("{:#?} /test_request_endpoint HTTP/1.1", method).as_str());
        assert_eq!(binding.lines().next().unwrap(), "HTTP/1.1 404 Not Found")
    }
}
