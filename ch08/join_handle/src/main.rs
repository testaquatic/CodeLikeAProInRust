use tokio::runtime::Handle;

fn not_an_async_fuction(handel: Handle) {
    handel.block_on(async {
        println!("Second print statement");
    })
}

#[tokio::main]
async fn main() {
    println!("First print statement");
    let handle = Handle::current();
    std::thread::spawn(move || {
        not_an_async_fuction(handle);
    });
}
