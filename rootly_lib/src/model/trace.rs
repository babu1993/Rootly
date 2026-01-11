#[derive(Debug)]
pub struct Trace {
    trace_id: String,
    span_id: String,
    parent_span_id: String,
    name: String,
    start_time: u64,
    end_time: u64,
    events: Vec<String>,
}

impl Trace {
    pub fn new(trace_id: String, span_id: String, parent_span_id: String, name: String, start_time: u64, end_time: u64) -> Self {
        Trace {
            trace_id,
            span_id,
            parent_span_id,
            name,
            start_time,
            end_time,
            events: Vec::new(),
        }
    }
    
    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }
}