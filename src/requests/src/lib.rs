use std::collections::HashMap;

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
    match http_map.get(key){
        Some(value) => value.clone(),
        None => HTTPResponse::new(404, "Not Found")
    }
}
