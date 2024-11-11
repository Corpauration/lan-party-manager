use lpmng_mq::client::Client;

#[tokio::main]
async fn main() {
    let mut c = Client::connect("http://[::1]:8080").await.unwrap();
    let mut i = 0;
    loop {
        if c.ping().await {
            i += 1;
            println!("{}", i);
        }
    }
}
