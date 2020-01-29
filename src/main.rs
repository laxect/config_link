#[async_std::main]
async fn main() -> config_link::error::Result<()> {
    config_link::config_link().await?;
    Ok(())
}
