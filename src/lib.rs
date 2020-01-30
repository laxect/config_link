use async_std::{fs, path};
use structopt::StructOpt;

mod config;
pub mod error;

const CONFIG_FILE: &str = "link.toml";

#[derive(StructOpt, Debug)]
pub(crate) enum Opt {
    Init,
    Link,
}

pub(crate) async fn read_config<P: AsRef<path::Path>>(path: P) -> error::Result<config::Config> {
    let content = fs::read(path).await?;
    let config: config::Config = toml::from_slice(&content)?;
    Ok(config)
}

pub async fn link() -> error::Result<()> {
    let config = read_config(CONFIG_FILE).await?;
    config.do_all().await?;
    Ok(())
}

pub async fn init() -> error::Result<()> {
    let blank_config = config::Config::new();
    let config_str = toml::to_string(&blank_config)?;
    fs::write(CONFIG_FILE, config_str).await?;
    Ok(())
}

pub async fn read_option() -> error::Result<()> {
    match Opt::from_args() {
        Opt::Link => {
            link().await?;
        }
        Opt::Init => {
            init().await?;
        },
    };
    Ok(())
}
