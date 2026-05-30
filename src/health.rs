/// Returns the response body for the `/healthz` endpoint.
pub fn response_body() -> &'static str {
    r#"{"status":"ok"}"#
}

pub const PATH: &str = "/healthz";
