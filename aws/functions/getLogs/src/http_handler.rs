use lambda_http::{Body, Error, Request, Response};
use serde_json::json;
use layers::prelude::*;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let response = json!({});
    let resp = build_response(RootlyResponseStatus::success(), response);
    Ok(resp)
}
