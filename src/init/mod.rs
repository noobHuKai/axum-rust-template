use anyhow::Result;

use crate::model::Config;
mod config;
mod tracing;

pub fn init() -> Result<Config> {
    let cfg = config::init_config()?;

    tracing::init_tracing(&cfg)?;
    Ok(cfg)
}
