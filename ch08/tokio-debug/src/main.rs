use std::time::Duration;

use tokio::time::sleep;

#[tracing::instrument]
async fn sleep_1s() {
    sleep(Duration::from_secs(1)).await;
}

#[tracing::instrument]
async fn sleep_2s() {
    sleep(Duration::from_secs(2)).await;
}

#[tracing::instrument]
async fn sleep_3s() {
    sleep(Duration::from_secs(3)).await;
}

#[tokio::main]
async fn main() {
    console_subscriber::init();

    loop {
        tokio::spawn(sleep_1s());
        tokio::spawn(sleep_2s());
        sleep_3s().await;
    }
}

#[tokio::test]
async fn sleep_test() {
    use tokio::time::Instant;

    let start_time = Instant::now();
    sleep(Duration::from_secs(1)).await;
    let end_time = Instant::now();
    let seconds = end_time
        .checked_duration_since(start_time)
        .unwrap()
        .as_secs();
    assert_eq!(seconds, 1);
}
