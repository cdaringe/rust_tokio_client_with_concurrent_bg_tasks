use client::Client;

mod client;
mod refresher_thing;

#[tokio::main]
async fn main() {
    {
        let client = Client::new();
        let mut i = 5;
        while i > 0 {
            println!("client query result: {}", { &client.query().await });
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            i -= 1;
        }
    }

    println!("everything should stop running now. you better not see nothin below");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
}
