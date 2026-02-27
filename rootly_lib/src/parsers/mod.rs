use crate::model::prelude::*;
use opentelemetry_proto::tonic::collector::logs::v1::ExportLogsServiceRequest;
use opentelemetry_proto::tonic::collector::trace::v1::ExportTraceServiceRequest;

pub fn logs_parser(logs_request: ExportLogsServiceRequest) -> Vec<Log> {
    let mut logs:Vec<Log> = vec![];
    for resource_log in logs_request.resource_logs.iter() {
        if resource_log.scope_logs.is_empty() {
            continue;
        }
        for scope_log in resource_log.scope_logs.iter() {
            for log_record in scope_log.log_records.iter() {
                let severity = log_record.severity_number;
                let trace_id = format!("{:016x}", u128::from_be_bytes(log_record.trace_id.as_slice().try_into().ok().unwrap_or_default()));
                let span_id = format!("{:016x}", u64::from_be_bytes(log_record.span_id.as_slice().try_into().ok().unwrap_or_default()));
                let body = log_record.body.as_ref().unwrap();
                let observed_timestamp = log_record.observed_time_unix_nano / 1000;
                let timestamp = log_record.time_unix_nano / 1000;
                let log_level = log_record.severity_number;
                let event_name = log_record.event_name.clone();
                logs.push(Log::new(timestamp, observed_timestamp, log_level,
                                   event_name, body, trace_id, span_id))
            }
        }
    }
    logs
}

pub fn trace_parser(trace_request: ExportTraceServiceRequest) -> Vec<Trace> {
    let mut traces:Vec<Trace> = vec![];
    for resource_span in trace_request.resource_spans.iter() {
        if resource_span.scope_spans.is_empty() {
            continue;
        }
        for scope_span in resource_span.scope_spans.iter() {
            for span in scope_span.spans.iter() {
                let trace_id = format!("{:016x}", u128::from_be_bytes(span.trace_id.as_slice().try_into().ok().unwrap_or_default()));
                let span_id = format!("{:016x}", u64::from_be_bytes(span.span_id.as_slice().try_into().ok().unwrap_or_default()));
                let parent_span_id = format!("{:016x}", u64::from_be_bytes(span.parent_span_id.as_slice().try_into().ok().unwrap_or_default()));
                let name = span.name.clone();
                let start_time = span.start_time_unix_nano / 1000;
                let end_time = span.end_time_unix_nano / 1000;
                let mut trace = Trace::new(trace_id, span_id, parent_span_id, name, start_time, end_time);
                for event in span.events.iter() {
                    let event_name = event.name.clone();
                    trace.add_event(event_name);
                }
                traces.push(trace);
            }
        }
    }
    traces
}
