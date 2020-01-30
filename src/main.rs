#[async_std::main]
async fn main() -> config_link::error::Result<()> {
    config_link::read_option().await?;
    Ok(())
}
