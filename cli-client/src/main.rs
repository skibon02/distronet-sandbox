use std::io::stdin;
use client::DistronetClient;
use client::user::DistronetUser;
use simple_logger::SimpleLogger;
use tokio::task::block_in_place;

struct OnlineUser {
    name: String,
    root_mk: String,
}

async fn connect_to_user(client: &mut DistronetClient, selected_user: &OnlineUser) -> client::Result<()> {
    loop {
        println!(" 1>- Send message.");
        println!(" 0>- Exit.");

        if let Some(v) = tokio_read_usize().await {
            match v {
                1 => {
                    println!("Enter message:");
                    let message = tokio_readline().await;
                    client.send(format!(" {}: {}",selected_user.name, message)).await?;
                },
                0 => {
                    println!("Exiting...");
                    return Ok(());
                },
                _ => {
                    println!("Invalid input!");
                },
            }
        }
        else {
            println!("Invalid input!");
        }
    }
}

async fn select_user(client: &mut DistronetClient) -> client::Result<()> {
    println!("Getting users list from server...");

    // Some code to ask for online clients on server

    println!("Users online:");

    let users_online = vec![OnlineUser {
        name: String::from("User1"),
        root_mk: String::from("1")
    }, OnlineUser {
        name: String::from("User2"),
        root_mk: String::from("2")
    }, OnlineUser {
        name: String::from("User3"),
        root_mk: String::from("3")
    }];

    for (i,user_online) in users_online.iter().enumerate() {
        println!(" {}. {}", i+1, user_online.name);
    }
    println!(" 0. Exit");

    if let Some(v) = tokio_read_usize().await {
        if v == 0 || v > users_online.len() {
            println!("Exiting...");
            return Ok(());
        }
        else {
            let v = v - 1;
            println!("Connecting to user {}...", users_online[v].name);
            connect_to_user(client, &users_online[v]).await?;
        }
    }
    else {
        println!("Invalid input!");
    }


    Ok(())
}

async fn tokio_readline() -> String {
    let mut str = String::new();
    block_in_place(|| {
        stdin().read_line(&mut str).unwrap();
    });
    str
}

async fn tokio_read_usize() -> Option<usize> {
    let mut str = String::new();
    block_in_place(|| {
        stdin().read_line(&mut str).unwrap();
    });
    str.trim().parse().ok()
}

#[tokio::main]
async fn main() -> client::Result<()> {
    SimpleLogger::new().init().unwrap();

    // Take known user if exists
    let user = DistronetUser::restore();

    // Establish new connection to available server
    let mut client = DistronetClient::new_connection(user).await?;

    println!("Connection to server established!");

    loop {
        println!(" 1>- Select user.");
        println!(" 0>- Exit.");

        if let Some(v) = tokio_read_usize().await {
            match v {
                1 => {
                    select_user(&mut client).await?;
                },
                0 => {
                    println!("Exiting...");
                    client.finalize().await;
                    return Ok(());
                },
                _ => {
                    println!("Invalid input!");
                },
            }
        }
        else {
            println!("Invalid input!");
        }
    }
}
