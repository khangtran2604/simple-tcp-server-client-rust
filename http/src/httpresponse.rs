use std::{collections::HashMap, io::Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HTTPResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

// implement Default trait
impl<'a> Default for HTTPResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: Some(HashMap::new()),
            body: None,
        }
    }
}

// implement From trait
impl<'a> From<HTTPResponse<'a>> for String {
    fn from(value: HTTPResponse<'a>) -> Self {
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            value.version(),
            value.status_code(),
            value.status_text(),
            value.headers(),
            value.body().unwrap().len(),
            value.body().unwrap_or_default()
        )
    }
}

// implement new function (status_code: &'a str, headers: Option<...>, body: Option<..>) ->
// HTTPResponse<'a>
impl<'a> HTTPResponse<'a> {
    fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HTTPResponse<'a> {
        let mut default_response: HTTPResponse = HTTPResponse::default();

        default_response.status_code = status_code;
        default_response.body = body.clone();
        match status_code {
            "200" => {
                default_response.status_text = "OK";
                default_response.headers = headers;
            }
            "400" => {
                default_response.status_text = "INVALID_REQUEST";
            }
            "401" => {
                default_response.status_text = "UNAUTHORIZED";
            }
            "403" => {
                default_response.status_text = "FORBIDDEN";
            }
            "404" => {
                default_response.status_text = "Not Found";
            }
            _ => {
                default_response.status_text = "INTERNAL_SERVER_ERROR";
            }
        };

        if default_response.headers.is_none() {
            let mut _headers = HashMap::new();
            _headers.insert("Content-Type", "text/html");
            default_response.headers = Some(_headers);
        } else {
            let mut _headers = default_response.headers.clone().unwrap();
            _headers.insert("Content-Type", "text/html");
            default_response.headers = Some(_headers);
        }

        default_response
    }

    fn version(&self) -> &'a str {
        self.version
    }

    fn status_code(&self) -> &'a str {
        self.status_code
    }

    fn status_text(&self) -> &'a str {
        self.status_text
    }

    fn headers(&self) -> String {
        let mut headers_string = "".into();

        for (key, val) in self.headers.clone().unwrap().iter() {
            headers_string = format!("{}{}: {}\r\n", headers_string, key, val);
        }

        headers_string
    }

    fn body(&self) -> Option<String> {
        self.body.clone()
    }

    fn send_response(&self, write_stream: &mut impl Write) -> std::io::Result<()> {
        let response: String = self.clone().into();

        write!(write_stream, "{}", response)?;
        write_stream.flush()?;

        Ok(())
    }
}

// implement send_response method (&self, write_stream: &mut impl Write) -> Result<()>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HTTPResponse::new(
            "200",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HTTPResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HTTPResponse::new(
            "404",
            None,
            Some("Item was shipped on 21st Dec 2020".into()),
        );
        let response_expected = HTTPResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HTTPResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("Item was shipped on 21st Dec 2020".into()),
        };
        let http_string: String = response_expected.into();
        let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 33\r\n\r\nItem was shipped on 21st Dec 2020";
        assert_eq!(http_string, response_actual);
    }
}
