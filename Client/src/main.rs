
/*
struct copy equal with ..
struct init with same name/parameter
struct tuple
anonymous struct
custom print for struct with #[derive(Debug)]
nullable types (option)
match
match w condition
if let
vector
vector of enum
hashmap
zip
iterators
backtrace
Result
File handling
iterator for each, map, collect, filter, enumerate

multiple Modules

*/

#[allow(non_snake_case, dead_code, unused_imports)]

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


#[path = "../../serialization.rs"]
mod serialization;
#[path = "../../networking.rs"]
mod networking;
#[path = "../../structures.rs"]
mod structures;

extern crate flatbuffers;

pub use crate::networking::game_networking::{send,recv};
pub use crate::structures::Structures;
pub use crate::serialization::Serialization::{unpack_data, ask_for_player, ask_for_game_data, send_game_score, message, ask_for_online_players};


use std::{error::Error};
use tokio::net::TcpStream;
use futures::future;
use std::time::{ Duration };
use tokio::time::delay_for;
use std::sync::{Arc};
use tokio::sync::Mutex;
use std::io;
use colored::*;
use rand::Rng;
use std::process::exit;

#[macro_use]
extern crate num_derive;

async fn get_login(out_arc: &mut Arc<Structures::OutgoingPackets>, in_arc : &mut Arc<Structures::IncomingPackets>) -> Structures::Player {

    let mut name: String = String::new();
    let mut password: String = String::new();
    let mut referral_str: String = String::new();
    

    println!("{}","Please insert your Login!".yellow());
    
    io::stdin().read_line(&mut name).expect("Expected string.");
    let name = name.trim().to_string();

    if name.len() < 3 || name.len() > 16 {
        panic!("{}{}","Name too short/long (3-16),".red()," try again".green().bold());
    }

    println!("Hello [ {} ] {}",&name.red().bold(),"Please insert your password!".yellow());
    
    io::stdin().read_line(&mut password).expect("Expected string.");
    let password = password.trim().to_string();

    if password.len() < 3 || password.len() > 16{
        panic!("{}{}","Too short/long password (3-16),".red()," try again".green().bold());
    }  

    println!("Please {} insert your {} link! {}",&name.red().bold(),"referral".yellow(), "(ignore if already user)".bright_cyan());
    
    io::stdin().read_line(&mut referral_str).expect("Expected string.");
    let referral: i64 = match referral_str.trim().parse::<i64>() {
        Ok(t) => t,
        Err(_) => 0 
    };


    let mut outgoint_packets = Arc::clone(out_arc);
    let mut incoming_packets = Arc::clone(in_arc);

    loop {
        send_packet(&mut outgoint_packets, ask_for_player(&name, &password, referral)).await;
        delay_for(Duration::from_secs(1)).await;
        println!("Awaiting for login");
        let packets = receive_packets(&mut incoming_packets).await;

        for packet in packets.iter() {
            if let Structures::PacketTypes::AnswerPlayer{status, player} = packet {
                if let Structures::StatusAnswerPlayer::OkNew = status {
                    println!("{}","New player!".green());
                    
                    return Structures::Player{
                        name: player.name.to_string(),
                        auth_token: player.auth_token.to_string(),
                        password: player.password.to_string(),
                        score: player.score,
                        is_admin: player.is_admin,
                        referral: player.referral
                    };

                } else if let Structures::StatusAnswerPlayer::OkLogin = status{
                    println!("{}","Welcome back!".purple());

                    return Structures::Player{
                        name: player.name.to_string(),
                        auth_token: player.auth_token.to_string(),
                        password: player.password.to_string(),
                        score: player.score,
                        is_admin: player.is_admin,
                        referral: player.referral
                    };
                } else if let Structures::StatusAnswerPlayer::Denied = status{
                    println!("{}","Incorrect password".bold().red());
                    exit(-1);
                } else if let Structures::StatusAnswerPlayer::Failure = status{
                    println!("{}","Incorrect Referral link or invalid data".bold().red());
                    exit(-1);
                }
            }
        }       
        
    }
}


async fn get_game_data(player : &Structures::Player, out_arc: &mut Arc<Structures::OutgoingPackets>, in_arc : &mut Arc<Structures::IncomingPackets>) 
-> (u32,u32,u32){
    
    let mut outgoint_packets = Arc::clone(out_arc);
    let mut incoming_packets = Arc::clone(in_arc);

    loop {
        send_packet(&mut outgoint_packets, ask_for_game_data(&player)).await;
        delay_for(Duration::from_secs(1)).await;
        println!("Awaiting for game data");
        let packets = receive_packets(&mut incoming_packets).await;

        for packet in packets.iter() {
            if let Structures::PacketTypes::AnswerGameData{ motd, low,
                high, games} = packet {
                
                println!("{}\n{}", "---- ## Message of the day ## ----".red(),motd.green());
               
                return (*low,*high,*games);


            }
        }       
        
    }
}


async fn send_gamescore(match_score : &Structures::MatchScore, player : &Structures::Player,
     out_arc: &mut Arc<Structures::OutgoingPackets>){

    let mut outgoint_packets = Arc::clone(out_arc);

    /*println!("{}","Please inser your score message!".yellow());
    let mut message: String = String::new();
    io::stdin().read_line(&mut message).expect("Expected string.");*/
    let message = "".to_string();//message.trim().to_string();

    send_packet(&mut outgoint_packets, send_game_score(&player, &match_score, message)).await;
    delay_for(Duration::from_secs(1)).await;        

}


/*async fn receive_message(in_arc : &mut Arc<Structures::IncomingPackets>){
    let mut incoming_packets = Arc::clone(in_arc);

 
    let mut packets = receive_packets(&mut incoming_packets).await;


    packets.retain(|packet|{
        if let Structures::PacketTypes::Message{ text,  color, from} = packet {
            
            let message = match color {
                Structures::Color::Blue => text.blue(),
                Structures::Color::Green => text.green(),
                Structures::Color::Red => text.red()
            };

            println!("Message from: [{}] - {}", from.name, message);
    
            return false;
        }
        true
    })        
}*/


/*async fn send_message(player: &Structures::Player, message_p: String, out_arc: &mut Arc<Structures::OutgoingPackets>){
    let mut outgoint_packets = Arc::clone(out_arc);

    send_packet(out_arc, message(player, message_p, Structures::Color::Blue) ).await;       
}*/



fn get_guesses(low: u32, high: u32) -> Result<Vec<u32>,()> {
    
    let mut str_input = String::new();

    io::stdin().read_line(&mut str_input).expect("Expected string");

    let str_numbers : Vec<&str> = str_input.trim().split(" ").collect();

    if str_numbers.len() != 5 { 
        println!("{}{}","Oh no, i hope you can count to 5, like... just use your fingers bro,".red()," try again".green().bold());
        return Err(());
    }

    let mut guesses : Vec<u32> = Vec::new();

    for i in str_numbers.iter(){
        let n = i.parse();
        if let Err(_) = n {
            println!("Please insert only numbers");
            return Err(());
        }
        let n = n.unwrap();

        if n < low || n >= high {
            println!("{}{}",format!("Oh no no, the number should be bigger than {} and lower than {},",low,high).red().bold(),
            " try again".green().bold());
            return Err(());
        }
        guesses.push(n);
    }

    Ok(guesses)
}


fn check_guesses(guesses : &Vec<u32>, low: u32, high: u32, match_score : &mut Structures::MatchScore){
    let mut rng = rand::thread_rng();  


    let mut sum: i64 = match_score.score;

    for (i, &val) in guesses.iter().enumerate() {
        let rand = rng.gen_range(low,high);
        print!("{}", format!("Peek: G: {} P: {} ", val,rand).purple());
        if val ==  rand {
            println!("{} {} {}", "Guess".green(),i,"Hit!".blue().bold());
            match_score.hits += 1;
            sum += 111;
        } else if (val > 1)  && (rand % val == 0) {
            println!("{} {} {}", "Guess".green(),i,"Missed in a special way!".yellow().bold());
            match_score.specials += 1;
            sum += 31;
        } else {
            println!("{} {} {}", "Guess".green(),i,"Missed!".red().bold());
            match_score.misses += 1;
            sum -= 23;
        }   
    }

    println!("Current Score: {}",&sum);
    match_score.score = sum;

}

async fn receive_packets(packets : &mut Arc<Structures::IncomingPackets>) -> Vec<Structures::PacketTypes>{

    let mut processed_packets = Vec::new();
    let mut received_packets = packets.data.lock().await;
    //println!("Size {:?}",received_packets.len());
    for packet in received_packets.iter() {
        processed_packets.push(unpack_data(packet));
        //println!("Received: {:?}",&processed_packets[processed_packets.len()-1]);
    }
    received_packets.clear();
    processed_packets
}

async fn send_packet(packets_arc: &mut Arc<Structures::OutgoingPackets>, data: bytes::Bytes){
    let mut packets = packets_arc.data.lock().await;
    packets.push(data);    
}


async fn online_players(out_arc: &mut Arc<Structures::OutgoingPackets>, in_arc : &mut Arc<Structures::IncomingPackets>){
    let mut outgoint_packets = Arc::clone(out_arc);
    let mut incoming_packets = Arc::clone(in_arc);

    let mut rng = rand::thread_rng(); 

    loop {
        
        send_packet(&mut outgoint_packets, ask_for_online_players(rng.gen_range(0,9999999))).await;
        delay_for(Duration::from_secs(1)).await;
        println!("Awaiting for Online Players!");
        let mut packets = receive_packets(&mut incoming_packets).await;

        for packet in packets.iter_mut() {
            if let Structures::PacketTypes::AnswerOnlinePlayers{players} = packet {
                println!("{}\n","\n\nOther great people are also playing! check them out!\n".bold().bright_purple());
                println!("{}\t{}\n---------------------------------------------------------------",
                 "Name            ".bold().red(),"Score".bold().red());
                for player in players.iter_mut(){     
                    
                    let len_spce = 16 - player.name.len();

                    //another horribe algo right here
                    for _ in 0..len_spce-1 {
                        player.name.push(' ');
                    }

                    if len_spce > 0 {
                        player.name.push(' ');
                        player.name.push('-');
                    }


                    let score_colored = if player.score < 0 {
                        player.score.to_string().bold().red()
                    } else {
                        player.score.to_string().bold().green()
                    };

                    if player.is_admin {
                        println!("{}\t{}\t{}",player.name.bold().bright_blue(),
                        score_colored,"[ADMIN]".bold().bright_green());
                    }else{
                        let mut nice: String = String::new();
                        if player.score == 4321 {
                            nice += "[4/4 FLGS GJ!]";
                        }else if player.score == 2776 {
                            nice += "[3/4 FLGS]";
                        }

                        println!("{}\t{}\t{}",player.name.bold().bright_blue(),score_colored,nice.bright_purple().bold());
                    }                  
            
                }
                println!("\n");


               return;
            }
        }       
        
    }


}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    //------ NETWORKING


    let incoming : Structures::Packets = Vec::new();
    let mut incoming_packets = Arc::new(Structures::IncomingPackets {
        data: Mutex::new(incoming)
    });

    let outgoing :  Structures::Packets = Vec::new();
    let mut outgoint_packets = Arc::new(Structures::OutgoingPackets {
        data: Mutex::new(outgoing)
    });
    let mut thread_incoming_packets = Arc::clone(&incoming_packets);
    let mut thread_outgoint_packets = Arc::clone(&outgoint_packets);

    tokio::spawn(async move {

        let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
        
        let (mut r, mut w) = stream.split();
        let mut rarc = Arc::new(Mutex::new(& mut r));
        let mut warc = Arc::new(Mutex::new(& mut w)); 
    
        future::try_join(send(warc.clone(), &mut thread_outgoint_packets), recv(rarc.clone(), &mut thread_incoming_packets)).await;
    });   


    println!("\nWelcome to the Online {} Gambling game!", "Hackable".black());
    println!("{}","Notes: please don't spam the server, have fun!".blue());


    online_players(&mut outgoint_packets, &mut incoming_packets).await;
    
    let player = get_login(&mut outgoint_packets,&mut incoming_packets).await;
    let (low,high,games) = get_game_data(&player, &mut outgoint_packets,&mut incoming_packets).await;

    if player.referral > 0 {
        println!("\n{}: {}","Your Referral value is".bold().blink().green(), &player.referral);
    }

    //game_networking::ask_for_player(&name, &password);

    println!("\n\n {}, {}","Hello".green(),player.name.red());
    println!("{} {} {}","We will play".yellow() ,games,"rounds of Guess the number! (With special numbers)".yellow());
    println!("For each round, you have to guess 5 random numbers from {} in sequence,
     \ntry guessing by inputting all 5 numbers separated by space!", format!("{}-{}",low,high-1).red().bold());


    //GameNetworking::send_message(&player, String::from("Wow"), Structures::Color::Red);
    
    let mut count : u32 = 0;

    let mut match_score = Structures::MatchScore {
        hits : 0,
        specials: 0,
        misses: 0,
        score: 0,
    };

    while count < games {

        println!("{} {}","Ok, fire your 5 guesses for game: ".yellow(),count);
        let guesses = get_guesses(low,high);

        if let Err(_) = guesses {
            continue;
        }

        println!("You entered: {:?}", &guesses);
        check_guesses(&guesses.unwrap(), low, high, &mut match_score);

        //receive_message(&mut incoming_packets).await;
        //send_message(&player, "I'm Playing too!".to_string(), &mut outgoint_packets).await;

        count += 1;
    }

    match_score.score += if match_score.score % 2 == 0 { 0 } else { 
        1 
    };


    send_gamescore(&match_score, &player, &mut outgoint_packets).await;




    Ok(())

}
