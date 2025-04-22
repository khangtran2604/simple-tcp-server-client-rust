use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HTTPVersion {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for HTTPVersion {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Self::V1_1,
            _ => Self::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Path(String);

#[derive(Debug, PartialEq)]
pub struct HTTPRequest {
    pub method: Method,
    pub version: HTTPVersion,
    pub path: Path,
    pub headers: BTreeMap<String, String>,
    pub body: Option<String>,
}

fn get_request_line(request_line: &str) -> (Method, HTTPVersion, Path) {
    let mut parts = request_line.split_whitespace();

    let method: Method = parts.next().unwrap().into();
    let path: Path = Path(parts.next().unwrap().to_string());
    let version: HTTPVersion = parts.next().unwrap().into();

    (method, version, path)
}

fn get_headers(header_line: &str) -> (String, String) {
    let mut key = String::from("");
    let mut value = String::from("");

    for (ind, val) in header_line.split(":").enumerate() {
        if ind == 0 {
            key = val.to_string();
        } else if ind > 1 {
            value += format!(":{}", val).trim();
        } else {
            value += val.trim();
        }
    }

    (key, value)
}

impl From<&str> for HTTPRequest {
    fn from(value: &str) -> Self {
        let mut method = Method::Uninitialized;
        let mut version = HTTPVersion::Uninitialized;
        let mut path = Path("".to_string());
        let mut headers: BTreeMap<String, String> = BTreeMap::new();
        let mut body = None;

        for line in value.lines() {
            if line.contains("HTTP") {
                (method, version, path) = get_request_line(line);
            } else if line.contains(":") {
                let (key, value) = get_headers(line);
                headers.insert(key, value);
            } else if line.is_empty() {
                // empty line
            } else {
                body = Some(line.to_string());
            }
        }

        Self {
            method,
            version,
            path,
            headers,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        let get_method: Method = "GET".into();

        assert_eq!(get_method, Method::Get);
    }

    #[test]
    fn test_http_version() {
        let http_v1_1: HTTPVersion = "HTTP/1.1".into();

        assert_eq!(http_v1_1, HTTPVersion::V1_1);
    }

    #[test]
    fn test_http_request() {
        let request = "GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";

        let actual: HTTPRequest = request.into();

        assert_eq!(actual.method, Method::Get);
        assert_eq!(actual.path, Path("/greeting".to_string()));
        assert_eq!(actual.version, HTTPVersion::V1_1);

        let mut headers: BTreeMap<String, String> = BTreeMap::new();
        headers.insert("Host".to_string(), "localhost:3000".to_string());
        headers.insert("User-Agent".to_string(), "curl/7.64.1".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());

        assert_eq!(actual.headers, headers);
        assert_eq!(actual.body, None);
    }
}
