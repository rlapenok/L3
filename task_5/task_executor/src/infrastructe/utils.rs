/*use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use opentelemetry::{
    global::get_text_map_propagator, propagation::Extractor, trace::TraceContextExt, Context,
};


#[derive(Debug)]
pub struct HeaderMap(HashMap<String, String>);


impl HeaderMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl Deref for HeaderMap {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for HeaderMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Extractor for HeaderMap {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(|key| key.as_str())
    }
    fn keys(&self) -> Vec<&str> {
        self.0.keys().into_iter().map(|key| key.as_str()).collect()
    }
}*/

/*pub fn create_header_map(trace_id: &str, span_id: &str) -> HeaderMap {
    let trace_parent = format!("00-{}-{}-01", trace_id, span_id);
    let mut header_map = HeaderMap::new();

    header_map.insert("traceparent".to_owned(), trace_parent);
    header_map.insert("tracestate".to_owned(), "".to_owned());

    header_map
}

pub fn set_parent_otel_context(span: Span, trace_id: &str, prev_span_id: &str) {
    let header_map = create_header_map(trace_id, prev_span_id);

    let parent_context = get_text_map_propagator(|prop| prop.extract(&header_map));
    span.set_parent(parent_context);
}

pub fn get_parent_context(trace_id: &str, prev_span_id: &str) -> Context {
    let header_map = create_header_map(trace_id, prev_span_id);
    let parent_context = get_text_map_propagator(|prop| prop.extract(&header_map));
    parent_context
}

pub fn get_span_id(span: &Span) -> String {
    span.context().span().span_context().span_id().to_string()
}
pub fn get_trace_id(span: &Span) -> String {
    span.context().span().span_context().trace_id().to_string()
}*/
