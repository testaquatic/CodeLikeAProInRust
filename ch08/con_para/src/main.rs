use std::{thread, time::Duration};

async fn sleep_1s_blocking(task: &str) {
    println!("Entering sleep_1s_blocking({task})");
    thread::sleep(Duration::from_secs(1));
    println!("Returning from sleep_1s_blocking({task})");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("Test 1: Run 2 async tasks sequentially");
    sleep_1s_blocking("Task 1").await;
    sleep_1s_blocking("Task 2").await;

    println!("Test 2: Run 2 async tasks concurrently (same thread)");
    tokio::join!(sleep_1s_blocking("Task 3"), sleep_1s_blocking("Task 4"));

    println!("Test 3: Run 2 async tasks in parallel");
    let _ = tokio::join!(
        tokio::spawn(sleep_1s_blocking("Task 5")),
        tokio::spawn(sleep_1s_blocking("Task 6"))
    );
}
