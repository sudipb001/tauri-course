use tokio::time::{Duration, sleep};

async fn simulate_file_read() -> String {
    println!("Reading file...");
    sleep(Duration::from_secs(2)).await;
    println!("File read complete");
    String::from("report.csv")
}

#[tokio::main]
async fn main() {
    let file_name = simulate_file_read().await;
    println!("Got file: {}", file_name);
}
