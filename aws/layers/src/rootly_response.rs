use serde_json::Value;
use lambda_http::{Response as LambdaResponse, Body as LambdaResponseBody};

#[repr(u16)]
#[derive(Debug)]
pub enum RootlyResponseStatus {
    Status(u16),
}
impl RootlyResponseStatus {
    pub fn success() -> Self {
        RootlyResponseStatus::Status(200)
    }
    pub fn bad_request() -> Self {
        RootlyResponseStatus::Status(400)
    }
    pub fn conflict() -> Self {
        RootlyResponseStatus::Status(409)
    }
    pub fn created() -> Self {
        RootlyResponseStatus::Status(201)
    }
    pub fn error() -> Self {
        RootlyResponseStatus::Status(500)
    }
}
impl From<u16> for RootlyResponseStatus {
    fn from(status: u16) -> Self {
        match status {
            200 => RootlyResponseStatus::Status(status),
            400 => RootlyResponseStatus::Status(status),
            409 => RootlyResponseStatus::Status(status),
            201 => RootlyResponseStatus::Status(status),
            _ => RootlyResponseStatus::Status(status),
        }
    }
}

impl Into<u16> for RootlyResponseStatus {
    fn into(self) -> u16 {
        match self {
            RootlyResponseStatus::Status(status) => status,
        }
    }
}

pub fn build_response(status: RootlyResponseStatus, body:Value) -> LambdaResponse<LambdaResponseBody> {
    let status_code:u16 = status.into();
    let builder = LambdaResponse::builder()
        .status(status_code)
        .header("content-type", "application/json")
        .body(body.to_string().into())
        .map_err(Box::new);
    builder.unwrap()
}