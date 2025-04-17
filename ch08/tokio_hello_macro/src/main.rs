use tokio::time;

#[tokio::main]
async fn main() {
    let duration = time::Duration::from_secs(1);
    tokio::time::sleep(duration).await;
    println!("Hello, world!");
}
