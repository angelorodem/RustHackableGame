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

#[path = "../../../Flat_Modules/GenericPacket_generated.rs"]
mod GenericPacket_generated;

#[path = "../../../Flat_Modules/AnswerGameData_generated.rs"]
mod AnswerGameData_generated;
#[path = "../../../Flat_Modules/AnswerOnlinePlayers_generated.rs"]
mod AnswerOnlinePlayers_generated;
#[path = "../../../Flat_Modules/AnswerPlayer_generated.rs"]
mod AnswerPlayer_generated;

#[path = "../../../Flat_Modules/AskForGameData_generated.rs"]
mod AskForGameData_generated;
#[path = "../../../Flat_Modules/AskForOnlinePlayers_generated.rs"]
mod AskForOnlinePlayers_generated;
#[path = "../../../Flat_Modules/AskForPlayer_generated.rs"]
mod AskForPlayer_generated;

#[path = "../../../Flat_Modules/Message_generated.rs"]
mod Message_generated;
#[path = "../../../Flat_Modules/SendGameScore_generated.rs"]
mod SendGameScore_generated;

#[path = "../../../Flat_Modules/Player_generated.rs"]
mod Player_generated;

use std::{error::Error};
use std::sync::{Arc};
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use futures::{future};
use bytes::Bytes;

use std::io;
use std::time::{ Instant, Duration };
use tokio::time::delay_for;



extern crate flatbuffers;


#[path = "../../../serialization.rs"]
mod serialization;
#[path = "../../../networking.rs"]
mod networking;
#[path = "../../../structures.rs"]
mod structures;

pub use crate::networking::game_networking::{send,recv};
pub use crate::structures::Structures;
pub use crate::serialization::Serialization::{unpack_data, ask_for_player};



#[macro_use]
extern crate num_derive;

async fn handle_receive_packets(mut packets : &mut Arc<Structures::IncomingPackets>){

    let mut received_packets = packets.data.lock().await;
    println!("Size {:?}",received_packets.len());
    for packet in received_packets.iter() {
        let ret = unpack_data(packet);
        println!("Server: {:#?}",ret);
    }
    received_packets.clear();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut inc : Structures::Packets = Vec::new();
    inc.push(ask_for_player(&"angelo".to_string(), &"abc".to_string()));

    let mut incoming_packets = Arc::new(Structures::IncomingPackets {
        data: Mutex::new(inc)
    });

    let mut outg :  Structures::Packets = Vec::new();
    outg.push(ask_for_player(&"angelo".to_string(), &"abc".to_string()));
    let mut outgoint_packets = Arc::new(Structures::OutgoingPackets {
        data: Mutex::new(outg)
    });

    let mut local_incoming_packets = Arc::clone(&incoming_packets);
    let mut local_outgoint_packets = Arc::clone(&outgoint_packets);

    tokio::spawn(async move {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        let (mut r, mut w) = stream.split();
        let mut rarc = Arc::new(Mutex::new(& mut r));
        let mut warc = Arc::new(Mutex::new(& mut w)); 
    
        future::try_join(send(warc.clone(), &mut local_outgoint_packets), recv(rarc.clone(), &mut local_incoming_packets)).await;
    });

    loop {
        delay_for(Duration::from_secs(1)).await;
        println!("At same time");

        handle_receive_packets(&mut incoming_packets).await;

    }




    Ok(())
}

