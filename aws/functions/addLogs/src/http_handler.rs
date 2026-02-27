use base64::prelude::*;
use lambda_http::{Body, Error, Request, Response};
use layers::prelude::*;
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use prost::Message;
use serde_json::json;

const PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";
const JSON_CONTENT_TYPE: &str = "application/json";

pub async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let content_type = event.headers().get("content-type").and_then(|ct| ct.to_str().ok()).unwrap_or("");
    if content_type == PROTOBUF_CONTENT_TYPE {
        let body_bytes = event.body().as_ref();
        let body_bytes = BASE64_STANDARD.decode(body_bytes).unwrap_or_default();
        let request = ExportLogsServiceRequest::decode(body_bytes.as_ref())?;
        println!("{:#?}", request.resource_logs);
        // rootly.write_log(request);
    }
    else if content_type == JSON_CONTENT_TYPE {
        let body_bytes = event.body().as_ref();
        println!("Received trace data ({} bytes)", body_bytes.len());
    }
    let response = json!({});
    let resp = build_response(RootlyResponseStatus::success(), response);
    Ok(resp)
}

