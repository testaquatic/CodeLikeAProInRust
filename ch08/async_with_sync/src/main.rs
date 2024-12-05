use std::io;

use tokio::io::AsyncWriteExt;

async fn write_file(filename: &str) -> Result<(), io::Error> {
    let mut f = tokio::fs::File::create(filename).await?;
    f.write(b"Hello, file!").await?;

    Ok(())
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(filename)
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let filename = "mixed-sync-async.txt";
    write_file(&filename).await?;

    let contents = tokio::task::spawn_blocking(|| read_file(filename)).await??;
    println!("File contents: {}", contents);

    tokio::fs::remove_file(filename).await?;

    Ok(())
}
