use client::{DistronetClient};
use client::user::DistronetUser;
use simple_logger::SimpleLogger;

fn main() -> client::Result<()> {
    let logger = SimpleLogger::new().init();


    // Take known user if exists
    let user = DistronetUser::restore();
    // Establish new connection to available server
    let mut client = DistronetClient::new_connection(user)?;
    client.send("hello")?;

    Ok(())
}
