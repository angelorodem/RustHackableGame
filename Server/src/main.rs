#[allow(non_snake_case, dead_code, unused_imports)]

#[macro_use]
extern crate num_derive;

#[path = "../../serialization.rs"]
mod serialization;
#[path = "../../networking.rs"]
mod networking;
#[path = "../../structures.rs"]
mod structures;

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
#[path = "../../Flat_Modules/AnswerPlayer_generated.rs"]
mod AnswerPlayer_generated;
#[path = "../../Flat_Modules/SendPlayerGameScore_generated.rs"]
mod SendPlayerGameScore_generated;
#[path = "../../Flat_Modules/GenericPacket_generated.rs"]
mod GenericPacket_generated;

extern crate flatbuffers;

pub use crate::networking::game_networking::{parse_packets_stream, prepare_data};
pub use crate::structures::Structures;

use std::sync::{Arc,Mutex};

use futures::SinkExt;
use tokio::net::TcpStream;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, BytesCodec};

use tokio::net::TcpListener;
use tokio::prelude::*;

use internet_checksum::*;

use bytes::Bytes;

use std::time::Duration;
use async_std::task;

pub use crate::serialization::Serialization::{unpack_data, ask_for_player, answer_player};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    // println!("Listening on: {}", addr);


    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {      

                //let mut shared_data = sharedv.clone();                

                tokio::spawn(async move {           
                    let mut frame = Framed::new(socket, BytesCodec::new());
                    
                    while let Some(data) = frame.next().await {
                        match data {
                            Ok(mut byte) => {
                                //println!("{:?}",byte);
                                let n = byte.len();
                                let mut packets : Structures::Packets = Vec::new();
                                if let Err(t) = parse_packets_stream(&mut byte[..],n, &mut packets) {
                                    println!("Decoding error");
                                }

                                let mut to_send = handle_packets(&mut packets).await;

                                while !to_send.is_empty() {
                                    println!("Enviando : {:#?}", &to_send);
                                    let mut data = to_send.pop().unwrap();
                                    let packet_ready = prepare_data(&mut data);
                                    if let Err(e) = frame.send(packet_ready).await {
                                        println!("error on sending response; error = {:?}", e);
                                    }
                                }


                            },
                            Err(e) => println!("{:?}",e),
                        }
                    }                    
                });
            },
            Err(e) => println!("Error reading socket {:?}", e),
        }
        task::sleep(Duration::from_millis(2)).await;
    }
}


async fn handle_packets(received_packets : &mut Structures::Packets) -> Structures::Packets{
    println!("Size {:?}",received_packets.len());

    let mut packets_to_send : Structures::Packets = Vec::new();

    for packet in received_packets.iter() {
        let ret = unpack_data(packet);
        println!("{:#?}",ret);

        let ret : bytes::Bytes = match ret {
            Structures::PacketTypes::AskForPlayer{name, password} => {
                ask_for_player_handle(name,password).await
            }
            Structures::PacketTypes::GameData{ motd, low , high, games} => {
                bytes::Bytes::from("not supposed to recieve")
            }
            Structures::PacketTypes::Message{text,  color, from} => {
                bytes::Bytes::from("--wip--")
            }
            Structures::PacketTypes::OnlinePlayers{players} => {
                bytes::Bytes::from("not supposed to recieve")
            }
            Structures::PacketTypes::AnswerPlayer{status, player} => {
                bytes::Bytes::from("not supposed to recieve")
            }
            Structures::PacketTypes::SendGameScore{player, game_result, score_message} => {
                bytes::Bytes::from("--wip--")
            }
            Structures::PacketTypes::None => {
                bytes::Bytes::from("not supposed to recieve")
            }
        };

        if !ret.is_empty() {
            packets_to_send.push(ret);
        }   

    }
    received_packets.clear();

    packets_to_send
}


async fn ask_for_player_handle(name: String, password: String) -> bytes::Bytes {    

    //add some condition
    let new_player = Structures::Player{
        name,
        password,
        auth_token: "mudar_esse_authtoken".to_string(),
        score: 0,
        is_admin: false
    };
    answer_player(&new_player, &Structures::StatusAnswerPlayer::OkNew)    
}
