use crate::model::Config;

pub fn init_config() -> anyhow::Result<Config> {
    let config_str = std::fs::read_to_string("conf/config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
