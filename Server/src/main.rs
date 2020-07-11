#[warn(non_snake_case)]

#[path = "../../Flat_Modules/AskForPlayer_generated.rs"]
mod AskForPlayer_generated;
#[path = "../../Flat_Modules/GameData_generated.rs"]
mod GameData_generated;
#[path = "../../Flat_Modules/GameResult_generated.rs"]
mod GameResult_generated;
#[path = "../../Flat_Modules/Message_generated.rs"]
mod Message_generated;
#[path = "../../Flat_Modules/OnlinePlayers_generated.rs"]
mod OnlinePlayers_generated;
#[path = "../../Flat_Modules/Player_generated.rs"]
mod Player_generated;
#[path = "../../Flat_Modules/ReceivePlayer_generated.rs"]
mod ReceivePlayer_generated;
#[path = "../../Flat_Modules/SendPlayerGameScore_generated.rs"]
mod SendPlayerGameScore_generated;
#[path = "../../Flat_Modules/GenericPacket_generated.rs"]
mod GenericPacket_generated;

#[path = "../../networking.rs"]
mod networking;
#[path = "../../structures.rs"]
mod structures;

extern crate flatbuffers;
pub use crate::structures::Structures;
pub use crate::networking::GameNetworking;


use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 64];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                
                println!("{:?}",&buf[..]);
                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}