use std::collections::HashMap;

pub struct HTTPResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

// implement Default trait

// implement new function (status_code: &'a str, headers: Option<...>, body: Option<..>) ->
// HTTPResponse<'a>

// implement send_response method (&self, write_stream: &mut impl Write) -> Result<()>

// implement getter methods

// implement From trait

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn test_response_struct_creation_200() {
    //         let response_actual = HttpResponse::new(
    //             "200",
    //             None,
    //             Some("Item was shipped on 21st Dec 2020".into()),
    //         );
    //         let response_expected = HttpResponse {
    //             version: "HTTP/1.1",
    //             status_code: "200",
    //             status_text: "OK",
    //             headers: {
    //                 let mut h = HashMap::new();
    //                 h.insert("Content-Type", "text/html");
    //                 Some(h)
    //             },
    //             body: Some("Item was shipped on 21st Dec 2020".into()),
    //         };
    //         assert_eq!(response_actual, response_expected);
    //     }
    //
    //     #[test]
    //     fn test_response_struct_creation_404() {
    //         let response_actual = HttpResponse::new(
    //             "404",
    //             None,
    //             Some("Item was shipped on 21st Dec 2020".into()),
    //         );
    //         let response_expected = HttpResponse {
    //             version: "HTTP/1.1",
    //             status_code: "404",
    //             status_text: "Not Found",
    //             headers: {
    //                 let mut h = HashMap::new();
    //                 h.insert("Content-Type", "text/html");
    //                 Some(h)
    //             },
    //             body: Some("Item was shipped on 21st Dec 2020".into()),
    //         };
    //         assert_eq!(response_actual, response_expected);
    //     }
    //
    //     #[test]
    //     fn test_http_response_creation() {
    //         let response_expected = HttpResponse {
    //             version: "HTTP/1.1",
    //             status_code: "404",
    //             status_text: "Not Found",
    //             headers: {
    //                 let mut h = HashMap::new();
    //                 h.insert("Content-Type", "text/html");
    //                 Some(h)
    //             },
    //             body: Some("Item was shipped on 21st Dec 2020".into()),
    //         };
    //         let http_string: String = response_expected.into();
    //         let response_actual = "HTTP/1.1 404 Not Found\r\nContent-Type:
    // text/html\r\nContent-Length: 33\r\n\r\nItem was
    // shipped on 21st Dec 2020";
    //         assert_eq!(http_string, response_actual);
    //     }
}
