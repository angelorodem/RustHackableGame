//! An example of hooking up stdin/stdout to either a TCP or UDP stream.
//!
//! This example will connect to a socket address specified in the argument list
//! and then forward all data read on stdin to the server, printing out all data
//! received on stdout. An optional `--udp` argument can be passed to specify
//! that the connection should be made over UDP instead of TCP, translating each
//! line entered on stdin to a UDP packet to be sent to the remote address.
//!
//! Note that this is not currently optimized for performance, especially
//! around buffer management. Rather it's intended to show an example of
//! working with a client.
//!
//! This example can be quite useful when interacting with the other examples in
//! this repository! Many of them recommend running this as a simple "hook up
//! stdin/stdout to a server" to get up and running.

#[allow(non_snake_case, dead_code, unused_imports)]

#[path = "../../../Flat_Modules/AskForPlayer_generated.rs"]
mod AskForPlayer_generated;
#[path = "../../../Flat_Modules/AnswerGameData_generated.rs"]
mod AnswerGameData_generated;
#[path = "../../../Flat_Modules/GameResult_generated.rs"]
mod GameResult_generated;
#[path = "../../../Flat_Modules/Message_generated.rs"]
mod Message_generated;
#[path = "../../../Flat_Modules/OnlinePlayers_generated.rs"]
mod OnlinePlayers_generated;
#[path = "../../../Flat_Modules/Player_generated.rs"]
mod Player_generated;
#[path = "../../../Flat_Modules/AnswerPlayer_generated.rs"]
mod AnswerPlayer_generated;
#[path = "../../../Flat_Modules/SendPlayerGameScore_generated.rs"]
mod SendPlayerGameScore_generated;
#[path = "../../../Flat_Modules/GenericPacket_generated.rs"]
mod GenericPacket_generated;

extern crate flatbuffers;

use std::{error::Error};
use tokio::net::TcpStream;
use bytes::Bytes;
use futures::future;

#[path = "../../../serialization.rs"]
mod serialization;
#[path = "../../../networking.rs"]
mod networking;
#[path = "../../../structures.rs"]
mod structures;

pub use crate::networking::game_networking::{send,recv};
pub use crate::structures::Structures;
pub use crate::serialization::Serialization::{unpack_data, ask_for_player};

use std::time::{ Instant, Duration };
use tokio::time::delay_for;

#[macro_use]
extern crate num_derive;

async fn handle_receive_packets(received_packets : &mut Structures::Packets){
    println!("Size {:?}",received_packets.len());
    for packet in received_packets.iter() {
        let ret = unpack_data(packet);
        println!("Server: {:#?}",ret);
    }
    received_packets.clear();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut in_half, mut out_half) = stream.split();

    //
    let mut send_packets : Structures::Packets = Vec::new();
    send_packets.push(ask_for_player(&"angelo".to_string(), &"abc".to_string()));
    //send_packets.push(Bytes::from("aa"));

    let mut receive_packets : Structures::Packets = Vec::new();

    loop {
        delay_for(Duration::from_secs(1)).await;
        //println!("{:?}",count);
        handle_receive_packets(&mut receive_packets).await;
        
        send(&mut out_half, &mut send_packets).await?;
        recv(&mut in_half,  &mut receive_packets).await?;
    }

    Ok(())
}

