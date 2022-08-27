use lambda_http::Response;
use serde_json::json;

pub fn success_response() -> lambda_http::http::Result<Response<String>> {
    success_response_with_body(json!({
        "result": "ok"
    }).to_string())
}

pub fn success_response_with_body(body: String) -> lambda_http::http::Result<Response<String>> {
    Response::builder()
        .status(200)
        .body(body)
}

pub fn bad_request_response(message: &str) -> lambda_http::http::Result<Response<String>> {
    Response::builder()
        .status(400)
        .body(message.to_string())
}

pub fn internal_server_error_response() -> lambda_http::http::Result<Response<String>> {
    Response::builder()
        .status(500)
        .body("Internal server error".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_response_with_200_for_success_response() {
        let result = success_response().unwrap();

        assert_eq!(result.status().as_str(), "200");
    }

    #[test]
    fn should_build_a_response_with_400_for_bad_request_response_with_given_message() {
        let result = bad_request_response("Example message").unwrap();

        assert_eq!(result.status().as_str(), "400");
        assert_eq!(result.body().as_str(), "Example message");
    }

    #[test]
    fn should_build_a_response_with_500_for_internal_server_error_response() {
        let result = internal_server_error_response().unwrap();

        assert_eq!(result.status().as_str(), "500");
    }
}
