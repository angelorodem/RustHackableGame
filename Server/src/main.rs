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

use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;

use serde::{Serialize, Deserialize};

pub use crate::serialization::Serialization::{unpack_data, ask_for_player, answer_player, answer_game_data, answer_online_players};

static st_low: u32 = 1;
static st_high: u32 = 10;
static st_games: u32 = 5;
static st_modt: &str = "Prizes in gold bars are now available to best players!";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    // println!("Listening on: {}", addr);

    let mut random = rand::thread_rng();  
    let rand: i64 = random.gen_range(1,92233720368547);
    let mut referal_code = Arc::new(Mutex::new(rand));




    let mut plr : HashMap<String, Structures::Player>;

    let contents = match OpenOptions::new().read(true).write(true).create(true).open("save.bin") {
        Ok(mut t) => {
            let mut contents = Vec::new();
            let num = match t.read_to_end(&mut contents) {
                Ok(n) => {n} ,
                Err(_) => {0}
            };

            plr = match bincode::deserialize(&contents[..]) {
                Ok(ret) => {ret},
                Err(e) => {
                    println!("Error Deserializing! {:?}",e);
                    HashMap::new()
                }
            };

            if plr.is_empty() {
                println!("Error Loading!");
                false
            }else{
                println!("Loaded!");
                true
            }
        },
        Err(_) => {
            println!("Error opening file!");
            plr = HashMap::new();
            false
        }
    };


    if !contents {
        println!("Loading defaults!");
        plr.insert("@OLEG".to_string(),Structures::Player{
            name: "@OLEG".to_string(),
            auth_token: "".to_string(),
            password: "StrongPassword20".to_string(),
            score: 50_001,
            is_admin: true,
            referral: rand
        });

        plr.insert("Tommy Turbo".to_string(),Structures::Player{
            name: "Tommy Turbo".to_string(),
            auth_token: "".to_string(),
            password: "asdasdasd".to_string(),
            score: 123,
            is_admin: false,
            referral:0
        });

        plr.insert("Kiddo".to_string(),Structures::Player{
            name: "Kiddo".to_string(),
            auth_token: "".to_string(),
            password: "Kiddo9000".to_string(),
            score: -1051,
            is_admin: false,
            referral: 0
        });

        plr.insert("Mr.T".to_string(),Structures::Player{
            name: "Mr.T".to_string(),
            auth_token: "".to_string(),
            password: "IPityTheFools".to_string(),
            score: 936,
            is_admin: false,
            referral: 0
        });

        plr.insert("Hulk Hogan".to_string(),Structures::Player{
            name: "Hulk Hogan".to_string(),
            auth_token: "".to_string(),
            password: "Brotthar".to_string(),
            score: 873,
            is_admin: false,
            referral: 0
        });

        plr.insert("Highlander".to_string(),Structures::Player{
            name: "Highlander".to_string(),
            auth_token: "".to_string(),
            password: "ReallyLongPassword".to_string(),
            score: 666,
            is_admin: false,
            referral: 0
        });

        match bincode::serialize(&plr) {
            Ok(t) => {
                let mut handle = OpenOptions::new().read(true).write(true).
                                            create(true).open("save.bin").unwrap();
                handle.write_all(&t[..]).unwrap();
            },
            Err(_) => {
                println!("Could not serialize");
            }

        }
    }



    let shared_players = Arc::new(Mutex::new(plr));

    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {      

                //let mut shared_data = sharedv.clone();     

                let mut shared_referal_code = Arc::clone(&referal_code);
                let mut local_shared_players = Arc::clone(&shared_players);

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

                                let mut to_send = handle_packets(&mut packets, &mut shared_referal_code, &mut local_shared_players).await;
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
        //println!("{:#?}",ret);

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
        let mut plr_c = plr.clone();
        plr_c.referral = 0;
        plr_c.auth_token = "".to_string();


        //Platinando
        plr_c.score = if plr_c.score >= 2776 {
            if plr_c.password.len() > 16 {
                4321
            }else{
                plr_c.score
            }
        } else {
            plr_c.score
        };


        sorted.push(plr_c);
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
    let mut save = false;
    match players.get_mut(&player.name) {
        Some(mut plr) => {
            let mut validate: i64 = (game_result.hits as i64) * 111 + (game_result.specials as i64) * 31 - (game_result.misses as i64) * 23 ;

            validate += if validate % 2 == 0 {0} else {1};

            if validate != game_result.score || game_result.hits > st_high*5 {
                println!("Fake Result for {} -> Validate: {}  Received: {} RHits: {}",&player.name,validate, game_result.score, game_result.hits);
            } else{                
                if player.auth_token == plr.auth_token{
                    if plr.score < game_result.score {
                        println!("New highscore for: {} -> {}",&player.name,&game_result.score);

                        //Save to file when someone hits max
                        if game_result.score >= 2776 {
                            save = true;
                        }

                        
                        plr.score = game_result.score;
                    }
                } else{
                    println!("Wrong token");
                }
            }
        },
        None => {}
    };

    if save {
        match bincode::serialize(&*players) {
            Ok(t) => {
                let mut handle = OpenOptions::new().read(true).write(true).
                                            create(true).open("save.bin").unwrap();
                handle.write_all(&t[..]).unwrap();
            },
            Err(_) => {
                println!("Could not serialize");
            }

        }
    }

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

    if name.len() > 16 {
        return answer_player(&Structures::Player{
            ..Default::default()
        }, &Structures::StatusAnswerPlayer::Failure);
    }

    let mut save = false;

    let (ret_player, status) = match players.get_mut(&name) {
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
                plr.auth_token = val;
                let mut plr = plr.clone();                
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
                    score: -2000,
                    is_admin: false,
                    referral: 0
                };
                players.insert(name.clone(),new_plr.clone());
                save = true;
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

    match bincode::serialize(&*players) {
        Ok(t) => {
            let mut handle = OpenOptions::new().read(true).write(true).
                                        create(true).open("save.bin").unwrap();
            handle.write_all(&t[..]).unwrap();
        },
        Err(_) => {
            println!("Could not serialize");
        }

    }

    //add some condition

    answer_player(&ret_player, &status)    
}
