use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let response = client::connect("127.0.0.1:6379").await;
    let mut client = response.expect("Error connecting to Redis");

    let result = client.get("frank").await?;

    println!("connected and got value: {:?}", result);
    Ok(())
}
