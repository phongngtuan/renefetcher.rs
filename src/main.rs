mod telegram;
use telegram::Telegram;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let telegram = Telegram::new();
    let resp = telegram.get_update().await?;
    println!("{:#?}", resp);
    Ok(())
}
