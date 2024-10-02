use std::path::PathBuf;

use confique::Config;

#[derive(Config, Debug)]
pub(crate) struct MigartionConfig {
    pub(crate) path: PathBuf,
}
