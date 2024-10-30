use opentelemetry::trace:: TraceContextExt;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;


pub fn get_span_id(span:&Span)->String{
    span.context().span().span_context().span_id().to_string()
}
pub fn get_trace_id(span:&Span)->String{
    span.context().span().span_context().trace_id().to_string()
}
