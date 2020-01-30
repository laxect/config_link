#[async_std::main]
async fn main() -> config_link::error::Result<()> {
    match config_link::read_option().await {
        Err(e) => {
            eprintln!("{}", e);
        }
        Ok(_) => {}
    }
    Ok(())
}
