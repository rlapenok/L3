use std::{error::Error, str::FromStr};

use confique::Config;

use opentelemetry::{global::set_error_handler, trace::TracerProvider, KeyValue};
use opentelemetry_otlp::{new_exporter, new_pipeline, WithExportConfig};
use opentelemetry_sdk::{
    runtime::Tokio,
    trace::{Config as TraceConfig, Tracer},
    Resource,
};
use tracing::{error, Level};
use tracing_opentelemetry::{layer as otel_layer, OpenTelemetryLayer};
use tracing_subscriber::{
    filter::{Directive, Filtered},
    fmt::Layer,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer as LayerTrait, Registry,
};

type OtelFiltered = Filtered<OpenTelemetryLayer<Registry, Tracer>, EnvFilter, Registry>;

#[derive(Config)]
pub(crate) struct TracingConfig {
    level_otel: String,
    level_stdout: String,
    url: String,
}

impl TracingConfig {
    //create otel layer
    fn create_otel_layer(&self) -> Result<OtelFiltered, Box<dyn Error>> {
        let filter = self.create_otel_filter()?;
        let url = &self.url;
        set_error_handler(|err| error!("Error while send trace to GrafanaTempo:{}", err))?;
        let key_value = vec![KeyValue::new("service.name", "TaskNotifier")];
        let resourse = Resource::new(key_value);
        let cfg = TraceConfig::default().with_resource(resourse);
        let exporter = new_exporter().tonic().with_endpoint(url);
        let tracer = new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(cfg)
            .install_batch(Tokio)?
            .tracer("TaskNotifierTracer");
        let layer = otel_layer().with_tracer(tracer).with_filter(filter);
        Ok(layer)
    }
    //create ffilter for stdout subscriber
    fn create_stdout_filter(&self) -> Result<EnvFilter, Box<dyn Error>> {
        let level = Level::from_str(&self.level_stdout)?;
        let filter = EnvFilter::from_default_env()
            .add_directive(Directive::from(level))
            .add_directive("tower=off".parse()?)
            .add_directive("axum=off".parse()?)
            .add_directive("h2=off".parse()?)
            .add_directive("hyper_util=off".parse()?)
            .add_directive("tonic=off".parse()?)
            .add_directive("tower_http=off".parse()?);
        Ok(filter)
    }
    //create filter for otel subscriber
    fn create_otel_filter(&self) -> Result<EnvFilter, Box<dyn Error>> {
        let level = Level::from_str(&self.level_otel)?;
        let filter = EnvFilter::from_default_env()
            .add_directive(Directive::from(level))
            .add_directive("tower=off".parse()?)
            .add_directive("axum=off".parse()?)
            .add_directive("h2=off".parse()?)
            .add_directive("hyper_util=off".parse()?)
            .add_directive("tonic=off".parse()?)
            .add_directive("tower_http=debug".parse()?);
        Ok(filter)
    }
    //create subscriber
    pub fn run_sub(&self) -> Result<(), Box<dyn Error>> {
        let otel_layer = self.create_otel_layer()?;
        let stdout_filters = self.create_stdout_filter()?;
        let stdout_layer = Layer::new()
            .with_file(false)
            .compact()
            .with_target(false)
            .with_filter(stdout_filters);
        let sub = Registry::default().with(otel_layer).with(stdout_layer);
        //set_text_map_propagator(TraceContextPropagator::new());
        sub.init();
        Ok(())
    }
}
