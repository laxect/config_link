use async_std::{fs, path};

mod config;
pub mod error;

pub(crate) async fn read_config<P: AsRef<path::Path>>(path: P) -> error::Result<config::Config> {
    let content = fs::read(path).await?;
    let config: config::Config = toml::from_slice(&content)?;
    Ok(config)
}

pub async fn config_link() -> error::Result<()> {
    let config = read_config("link.yaml").await?;
    config.do_all().await?;
    Ok(())
}
