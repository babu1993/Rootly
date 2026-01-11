use std::iter::Map;
use axum::body::{to_bytes, Body};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use prost::Message;
use rootly_lib::Rootly;
use std::sync::Arc;


const PROTOBUF_CONTENT_TYPE: &str = "application/x-protobuf";
const JSON_CONTENT_TYPE: &str = "application/json";

pub async fn handle_traces(
    header: HeaderMap,
    State(rootly_ref): State<Arc<Rootly>>,
    body: Body,
) -> Response {
    let content_type = header.get("content-type");
    if content_type.is_none() {
        return Response::builder().status(StatusCode::BAD_REQUEST).body(Body::empty()).unwrap()
    }
    let content_type = content_type.unwrap().to_str().unwrap_or("");
    if content_type == PROTOBUF_CONTENT_TYPE {
        let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
        let request = ExportTraceServiceRequest::decode(bytes).unwrap();
        // println!("Received proto trace data ({:?} bytes)", request)
        // write_trace_log(request);
        rootly_ref.write_trace_log(request);
    }
    else if content_type == JSON_CONTENT_TYPE {
        let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
        println!("Received trace data ({} bytes)", bytes.len());
    }
    Response::builder()
        .status(StatusCode::OK)
        .body(axum::body::Body::empty())
        .unwrap()
}

pub async fn handle_metrics(
    State(rootly): State<Arc<Rootly>>,
    body: Body,
) -> Response {
    let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
    println!("Received metric data ({} bytes)", bytes.len());
    Response::builder()
        .status(StatusCode::OK)
        .body(axum::body::Body::empty())
        .unwrap()
}

pub async fn handle_logs(
    header: HeaderMap,
    State(rootly): State<Arc<Rootly>>,
    body: Body,
) -> Response {
    let content_type = header.get("content-type");
    if content_type.is_none() {
        return Response::builder().status(StatusCode::BAD_REQUEST).body(Body::empty()).unwrap()
    }
    let content_type = content_type.unwrap().to_str().unwrap_or("");
    if content_type == PROTOBUF_CONTENT_TYPE {
        let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
        let request = ExportLogsServiceRequest::decode(bytes).unwrap();
        // println!("Received proto trace data ({:?} bytes)", request);
        // write_log(request);
        rootly.write_log(request);
    }
    else if content_type == JSON_CONTENT_TYPE {
        let bytes = to_bytes(body, usize::MAX).await.unwrap_or_default();
        println!("Received trace data ({} bytes)", bytes.len());
    }
    Response::builder()
        .status(StatusCode::OK)
        .body(axum::body::Body::empty())
        .unwrap()
}