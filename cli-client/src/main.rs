use client::DistronetClient;
use client::user::DistronetUser;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> client::Result<()> {
    SimpleLogger::new().init().unwrap();

    // Take known user if exists
    let user = DistronetUser::restore();

    // Establish new connection to available server
    let mut client = DistronetClient::new_connection(user).await?;
    client.send(String::from("hello")).await?;
    client.send(String::from("hello2")).await?;
    client.finalize().await;

    Ok(())
}
