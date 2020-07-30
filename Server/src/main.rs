#[allow(non_snake_case, dead_code, unused_imports)]

#[macro_use]
extern crate num_derive;

#[path = "../../serialization.rs"]
mod serialization;
#[path = "../../networking.rs"]
mod networking;
#[path = "../../structures.rs"]
mod structures;

#[path = "../../Flat_Modules/GenericPacket_generated.rs"]
mod GenericPacket_generated;

#[path = "../../Flat_Modules/AnswerGameData_generated.rs"]
mod AnswerGameData_generated;
#[path = "../../Flat_Modules/AnswerOnlinePlayers_generated.rs"]
mod AnswerOnlinePlayers_generated;
#[path = "../../Flat_Modules/AnswerPlayer_generated.rs"]
mod AnswerPlayer_generated;

#[path = "../../Flat_Modules/AskForGameData_generated.rs"]
mod AskForGameData_generated;
#[path = "../../Flat_Modules/AskForOnlinePlayers_generated.rs"]
mod AskForOnlinePlayers_generated;
#[path = "../../Flat_Modules/AskForPlayer_generated.rs"]
mod AskForPlayer_generated;

#[path = "../../Flat_Modules/Message_generated.rs"]
mod Message_generated;
#[path = "../../Flat_Modules/SendGameScore_generated.rs"]
mod SendGameScore_generated;

#[path = "../../Flat_Modules/Player_generated.rs"]
mod Player_generated;

extern crate flatbuffers;

pub use crate::networking::game_networking::{parse_packets_stream, prepare_data, parse_packets_stream_sync};
pub use crate::structures::Structures;

use std::collections::HashMap;

use std::sync::{Arc};
use tokio::sync::Mutex;

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

use rand::Rng;

pub use crate::serialization::Serialization::{unpack_data, ask_for_player, answer_player, answer_game_data, answer_online_players};

static st_low: u32 = 1;
static st_high: u32 = 6;
static st_games: u32 = 5;
static st_modt: &str = "Prizes in gold bars are now available to best players!";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    // println!("Listening on: {}", addr);

    let mut random = rand::thread_rng();  
    let rand: i64 = random.gen_range(1,92233720368547);
    let mut referal_code = Arc::new(Mutex::new(rand));

    let mut plr = HashMap::new();

    plr.insert("@OLEG".to_string(),Structures::Player{
        name: "@OLEG".to_string(),
        auth_token: "".to_string(),
        password: "StrongPassword20".to_string(),
        score: 50_000,
        is_admin: true,
        referral: rand
    });

    plr.insert("Tommy Turbo".to_string(),Structures::Player{
        name: "Tommy Turbo".to_string(),
        auth_token: "".to_string(),
        password: "abcqwe123".to_string(),
        score: 17,
        is_admin: false,
        referral:0
    });

    plr.insert("XxXDemonLordXxX".to_string(),Structures::Player{
        name: "XxXDemonLordXxX".to_string(),
        auth_token: "".to_string(),
        password: "SoyBoy9000".to_string(),
        score: -351,
        is_admin: false,
        referral: 0
    });


    let shared_players = Arc::new(Mutex::new(plr));

    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {      

                //let mut shared_data = sharedv.clone();     

                let mut shared_referal_code = Arc::clone(&referal_code);
                let mut shared_sorted_players = Arc::clone(&shared_players);

                tokio::spawn(async move {           
                    let mut frame = Framed::new(socket, BytesCodec::new());

                    
                    while let Some(data) = frame.next().await {
                        match data {
                            Ok(mut byte) => {
                                //println!("{:?}",byte);
                                let n = byte.len();
                                let mut packets : Structures::Packets = Vec::new();
                                if let Err(t) = parse_packets_stream_sync(&mut byte[..],n, &mut packets) {
                                    println!("Decoding error");
                                }

                                let mut to_send = handle_packets(&mut packets, &mut shared_referal_code, &mut shared_sorted_players).await;
                                while !to_send.is_empty() {
                                    //println!("Enviando : {:#?}", &to_send);
                                    let mut data = to_send.pop().unwrap();
                                    if data.is_empty() {
                                        continue;
                                    }
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


async fn handle_packets(received_packets : &mut Structures::Packets, referral_server: &mut Arc<Mutex<i64>>,
     players_arc: &mut Arc<Mutex<HashMap<String,Structures::Player>>>) -> Structures::Packets{
    //oprintln!("Size {:?}",received_packets.len());

    let mut packets_to_send : Structures::Packets = Vec::new();

    for packet in received_packets.iter() {
        let ret = unpack_data(packet);
        println!("{:#?}",ret);

        let ret : bytes::Bytes = match ret {
            Structures::PacketTypes::AskForPlayer{name, password,referral} => {
                let mut players_arc_clone = Arc::clone(players_arc);
                ask_for_player_handle(name,password,referral, &mut players_arc_clone, &referral_server).await
            }
            Structures::PacketTypes::AskForGameData{player} => {
                send_game_data().await
            }
            Structures::PacketTypes::AskForOnlinePlayers{sequence_p} => {
                let mut players_arc_clone = Arc::clone(players_arc);
                answer_onlineplayers(&mut players_arc_clone).await
            }

            Structures::PacketTypes::AnswerGameData{ motd, low , high, games} => {
                bytes::Bytes::from("not supposed to recieve")
            }
            Structures::PacketTypes::AnswerOnlinePlayers{players} => {
                bytes::Bytes::from("not supposed to recieve")
            }
            Structures::PacketTypes::AnswerPlayer{status, player} => {
                bytes::Bytes::from("not supposed to recieve")
            }

            Structures::PacketTypes::Message{text,  color, from} => {
                receive_message(text, color, from).await               
            }
            Structures::PacketTypes::SendGameScore{player, game_result, score_message} => {
                let mut players_arc_clone = Arc::clone(players_arc);
                recieve_score(player, game_result, score_message, &mut players_arc_clone).await
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

async fn answer_onlineplayers(players_arc: &mut Arc<Mutex<HashMap<String,Structures::Player>>>) -> bytes::Bytes{
    let players = players_arc.lock().await;

    let mut sorted = Vec::new();
    //warning of extremely bad code
    for plr in players.values().into_iter() {
        sorted.push(plr.clone());
    }

    sorted.sort_by(|a, b| b.score.cmp(&a.score));

    answer_online_players(&sorted)
}

async fn receive_message(text: String,  color: Structures::Color, from: Structures::Player)
-> bytes::Bytes{
    /* mut rng = rand::thread_rng();  
    let rand = rng.gen_range(0,9999999);

    let mut messages = shared_message.lock().await;

    messages.push(internal_message{
        text,
        from,
        color,
        rand: rand
    });*/


    bytes::Bytes::from("")
}

async fn recieve_score(player: Structures::Player, game_result : Structures::MatchScore, score_message: String,
    players_arc: &mut Arc<Mutex<HashMap<String,Structures::Player>>>) -> bytes::Bytes{

    //println!("New score from: {}\n{:#?}\n{}",player.name,game_result,score_message);
    let mut players = players_arc.lock().await;

    match players.get_mut(&player.name) {
        Some(mut plr) => {
            if player.auth_token == plr.auth_token{
                if plr.score < game_result.score {
                    plr.score = game_result.score;
                }
            } else{
                println!("Wrong token");
            }
        },
        None => {}
    };

    bytes::Bytes::from("")
}

async fn send_game_data() -> bytes::Bytes {
    answer_game_data(st_modt.to_string(),st_low, st_high, st_games)
}

async fn ask_for_player_handle(name: String, password: String,referral: i64,
     players_arc: &mut Arc<Mutex<HashMap<String,Structures::Player>>>, referral_server: &Arc<Mutex<i64>>) -> bytes::Bytes {    
    //TODO

    let mut players = players_arc.lock().await;
    let val = {
    let mut rng = rand::thread_rng();  
        rng.gen_range(0,9999999).to_string()
    };

    let (ret_player, status) = match players.get(&name) {
        Some(plr) => {

            if plr.password != password {
                (
                    Structures::Player{
                        ..Default::default()
                    },
                    Structures::StatusAnswerPlayer::Denied
                )
            }else{
                let refnum = referral_server.lock().await;
                let mut plr = plr.clone();
                plr.auth_token = val;
                if plr.is_admin {
                    plr.referral = *refnum;
                }
                (plr, Structures::StatusAnswerPlayer::OkLogin)
            }            
        },
        None => {
            let refnum = referral_server.lock().await;
            if *refnum == referral {

            let new_plr = Structures::Player{
                name: name.clone(),
                password,
                auth_token: val,
                score: 0,
                is_admin: false,
                referral: 0
            };
            players.insert(name.clone(),new_plr.clone());
            (new_plr, Structures::StatusAnswerPlayer::OkNew )
        } else {
            (
                Structures::Player{
                    ..Default::default()
                },
                Structures::StatusAnswerPlayer::Failure
            )
        }
        }
    };

    //add some condition

    answer_player(&ret_player, &status)    
}
