use lambda_http::{Body, Error, Request, Response};
use layers::prelude::*;
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use prost::Message;
use serde_json::json;
use rootly_lib::prelude::logs_parser;

const PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";
const JSON_CONTENT_TYPE: &str = "application/json";

pub async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let content_type = event.headers().get("content-type").and_then(|ct| ct.to_str().ok()).unwrap_or("");
    let request: Result<ExportLogsServiceRequest, Error>;
    if content_type == PROTOBUF_CONTENT_TYPE {
        let body = event.body();
        let body_as_ref = body.as_ref();
        request = ExportLogsServiceRequest::decode(body_as_ref).map_err(|err| {
            println!("Failed to decode protobuf body: {}", err);
            Error::from("Failed to decode protobuf body")
        });
    }
    else if content_type == JSON_CONTENT_TYPE {
        let body_bytes = event.body().as_ref();
        println!("Received trace data ({} bytes)", body_bytes.len());
        request = Err(Error::from("JSON content type is not supported yet"));
    }
    else{
        println!("Unsupported content type: {}", content_type);
        request = Err(Error::from("Unsupported content type"));
    }
    if request.is_ok(){
        let logs = logs_parser(request?);
        println!("{:?}", logs);
    }
    else {
        request?;
    }
    let response = json!({});
    let resp = build_response(RootlyResponseStatus::success(), response);
    Ok(resp)
}

