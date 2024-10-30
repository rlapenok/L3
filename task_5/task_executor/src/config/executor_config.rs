use confique::Config;

#[derive(Config)]
pub(crate) struct ExecutorConfig {
    pub(crate) num_workers: usize,
    pub(crate) url: String,
}
