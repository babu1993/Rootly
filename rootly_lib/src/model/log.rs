use opentelemetry_proto::tonic::common::v1::AnyValue;
use opentelemetry_proto::tonic::common::v1::any_value::Value;
use serde_json::{json, Value as JsonValue};
use crate::model::ReadableModel;
use crate::model::prelude::*;
use std::mem::size_of;

#[derive(Debug)]
pub struct Log {
    timestamp: u64,
    observed_timestamp: u64,
    log_level: i32,
    event_name: String,
    body: String,
    trace_id: String,
    span_id: String,
}

impl Log {
    fn get_value(any_value: &AnyValue) -> String {
        let val;
        let value = any_value.value.as_ref().unwrap();
        match value {
            Value::StringValue(string) => {
                val = string.to_string();
            }
            Value::BoolValue(boolean) => {
                val = boolean.to_string()
            }
            Value::IntValue(int_val) => {
                val = int_val.to_string()
            }
            Value::DoubleValue(float_val) => {
                val = float_val.to_string()
            }
            Value::ArrayValue(array_val) => {
                let mut val_vec = vec![];
                for item in array_val.values.iter() {
                    val_vec.push(Log::get_value(item));
                }
                val = format!("[{}]", val_vec.join(", "));
            }
            Value::KvlistValue(kvlist_val) => {
                let mut map = json!({});
                for kv in kvlist_val.values.iter() {
                    let key = &kv.key;
                    let value_str = Log::get_value(&kv.value.as_ref().unwrap());
                    map[key] = JsonValue::from(value_str);
                }
                val = map.to_string();
            }
            Value::BytesValue(bytes_val) => {
                val = String::from_utf8_lossy(bytes_val).to_string();
            }

        }
        val
    }

    pub fn new(timestamp: u64, observed_timestamp: u64, log_level: i32, event_name: String, body: &AnyValue,
    trace_id: String, span_id: String) -> Log {
        Log {
            timestamp,
            observed_timestamp,
            log_level,
            event_name,
            body: Log::get_value(&body),
            trace_id,
            span_id
        }
    }

}

impl ReadableModel for Log {
    fn from_bytes(data: &[u8]) -> Self {
        let mut offset = 0;

        let timestamp = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let observed_timestamp = u64::from_be_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        let log_level = i32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let event_name_len = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let event_name = String::from_utf8(data[offset..offset + event_name_len].to_vec()).unwrap();
        offset += event_name_len;

        let body_len = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let body = String::from_utf8(data[offset..offset + body_len].to_vec()).unwrap();
        offset += body_len;

        let trace_id_len = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let trace_id = String::from_utf8(data[offset..offset + trace_id_len].to_vec()).unwrap();
        offset += trace_id_len;

        let span_id_len = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
        offset += 4;
        let span_id = String::from_utf8(data[offset..offset + span_id_len].to_vec()).unwrap();

        Log {
            timestamp,
            observed_timestamp,
            log_level,
            event_name,
            body,
            trace_id,
            span_id
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size_in_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.observed_timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.log_level.to_be_bytes());
        let event_name_len = self.event_name.len() as u32;
        bytes.extend_from_slice(&event_name_len.to_be_bytes());
        bytes.extend_from_slice(self.event_name.as_bytes());
        let body_len = self.body.len() as u32;
        bytes.extend_from_slice(&body_len.to_be_bytes());
        bytes.extend_from_slice(self.body.as_bytes());
        let trace_id_len = self.trace_id.len() as u32;
        bytes.extend_from_slice(&trace_id_len.to_be_bytes());
        bytes.extend_from_slice(self.trace_id.as_bytes());
        let span_id_len = self.span_id.len() as u32;
        bytes.extend_from_slice(&span_id_len.to_be_bytes());
        bytes.extend_from_slice(self.span_id.as_bytes());
        bytes
    }

    fn size_in_bytes(&self) -> usize {
        size_of::<u64>() * 2 + size_of::<i32>() + self.event_name.len() + self.body.len() +
        self.trace_id.len() + self.span_id.len() + size_of::<u32>() * 4 // for string lengths
    }
}

pub struct LogEntry {
    length: usize,
    log: Log,
}

impl LogEntry {
    pub fn default(log: Log) -> Self {

        LogEntry {
            length: log.size_in_bytes() + size_of::<usize>(),
            log
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }
}

impl ReadableModel for LogEntry {
    fn from_bytes(data: &[u8]) -> Self {
        let length = u64::from_be_bytes(data[0..8].try_into().unwrap()) as usize;
        let log = Log::from_bytes(&data[8..length]);
        LogEntry {
            length,
            log
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size_in_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.log.to_bytes());
        bytes
    }

    fn size_in_bytes(&self) -> usize {
        size_of::<u32>() + self.log.size_in_bytes()
    }
}

pub struct LogFile {
    header: MutableHeader,
    logs: Vec<LogEntry>
}

impl LogFile {
    pub fn new() -> Self {
        let header = MutableHeader::new(
            Header::default(),
            0
        );
        LogFile {
            header,
            logs: Vec::new()
        }
    }

    pub fn add_log(&mut self, log: Log) {
        let log_entry = LogEntry::default(log);
        self.header.set_total_length(self.header.get_total_length() + log_entry.get_length());
        self.logs.push(log_entry);

    }
    
    pub fn get_header(&self) -> &MutableHeader {
        &self.header
    }
    
    pub fn get_logs(&self) -> &Vec<LogEntry> {
        &self.logs
    }
}

impl ReadableModel for LogFile {
    fn from_bytes(data: &[u8]) -> Self {
        let header = MutableHeader::from_bytes(&data[0..super::HEADER_SEGMENT_LENGTH]);
        let mut offset = header.size_in_bytes();
        let mut logs = Vec::new();

        while offset < data.len() {
            let length = u32::from_be_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            let log_entry = LogEntry::from_bytes(&data[offset..offset + length]);
            logs.push(log_entry);
            offset += length;
        }

        LogFile {
            header,
            logs
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.size_in_bytes());
        bytes.extend_from_slice(&self.header.to_bytes());
        for log_entry in &self.logs {
            bytes.extend_from_slice(&log_entry.to_bytes());
        }
        bytes
    }

    fn size_in_bytes(&self) -> usize {
        let mut size = self.header.size_in_bytes();
        for log_entry in &self.logs {
            size += log_entry.size_in_bytes();
        }
        size
    }
}

