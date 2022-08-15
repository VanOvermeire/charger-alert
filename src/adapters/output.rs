use lambda_http::Response;

// TODO tests

pub fn success_response() -> lambda_http::http::Result<Response<String>> {
    Response::builder().status(200).body("Ok".to_string())
}

pub fn bad_request_response(message: &str) -> lambda_http::http::Result<Response<String>> {
    Response::builder().status(400).body(message.to_string())
}

pub fn internal_server_error_response() -> lambda_http::http::Result<Response<String>> {
    Response::builder().status(500).body("Internal server error".to_string())
}
