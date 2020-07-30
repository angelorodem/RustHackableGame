#[allow(non_snake_case, dead_code, unused_imports)]
pub mod Serialization {

    use bytes::{Bytes};
    use internet_checksum::checksum;

    pub use crate::structures::Structures;

    use num_derive::{FromPrimitive, ToPrimitive};    
    use num_traits::{FromPrimitive, ToPrimitive};

    pub use crate::GenericPacket_generated::{GenericPacket, GenericPacketArgs,Data, get_root_as_generic_packet};

    pub use crate::AskForPlayer_generated::{AskForPlayer,AskForPlayerArgs, get_root_as_ask_for_player};
    pub use crate::AskForGameData_generated::{AskForGameData, AskForGameDataArgs, get_root_as_ask_for_game_data};
    pub use crate::AskForOnlinePlayers_generated::{AskForOnlinePlayers, AskForOnlinePlayersArgs, get_root_as_ask_for_online_players};
 
    pub use crate::AnswerPlayer_generated::{AnswerPlayer, AnswerPlayerArgs, get_root_as_answer_player, StatusAnswerPlayer};
    pub use crate::AnswerGameData_generated::{AnswerGameData, AnswerGameDataArgs, get_root_as_answer_game_data};
    pub use crate::AnswerOnlinePlayers_generated::{AnswerOnlinePlayers, AnswerOnlinePlayersArgs, get_root_as_answer_online_players};

    pub use crate::Message_generated::{Message, MessageArgs,Color, get_root_as_message};
    pub use crate::Player_generated::{Player, PlayerArgs, get_root_as_player};
    pub use crate::SendGameScore_generated::{SendGameScore, SendGameScoreArgs, GameResult, get_root_as_send_game_score};



    //Unpacks Generic Packets
    pub fn unpack_data(data : &bytes::Bytes) -> Structures::PacketTypes {
        
        //size + checksum
        if data.len() <= 6 {
            return Structures::PacketTypes::None;
        }
        
        let sum = &data[data.len()-2..data.len()];
        let true_data = &data[0..data.len()-2];
        let chksum_new = checksum(&true_data);        

        //dbg!("pkt-{:#?} real-{:#?}",&sum[..],&chksum_new[..]);

        let matches = sum.iter().zip(&chksum_new).all(
            |(a, b)| { *a == *b }
        );

        if !matches {
            //println!("Corrupt package");
            return Structures::PacketTypes::None;
        }

        let recived_packed = get_root_as_generic_packet(&true_data);
        // AskForPlayer, AskForGameData, AskForOnlinePlayers,AnswerGameData, AnswerPlayer, AnswerOnlinePlayers, SendGameScore, Message
        match recived_packed.data_type() {
            Data::AskForPlayer => {
                println!("Received: AskForPlayer");   
                let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap();  
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string(), referral: ask_for_player.referral()};
            },
            Data::AskForGameData => {
                println!("Received: AskForGameData");

                let ask_for_gamedata =   recived_packed.data_as_ask_for_game_data().unwrap(); 

                let player = ask_for_gamedata.player().unwrap();
                let player_struct = Structures::Player{                    
                    name: player.name().to_string(),
                    auth_token: player.auth_token().unwrap_or("").to_string(),
                    password: player.password().unwrap_or("").to_string(),
                    score: player.score(),
                    is_admin: player.is_admin(),
                    referral: player.referral()
                };


                return Structures::PacketTypes::AskForGameData{player:player_struct};
                    
            },
            Data::AskForOnlinePlayers =>  { //ok
                println!("Received: AskForOnlinePlayers");

                let answer_onlineplayers =  recived_packed.data_as_ask_for_online_players().unwrap();



                return Structures::PacketTypes::AskForOnlinePlayers{sequence_p: answer_onlineplayers.sequence()};

                /*let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap(); 
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};*/
            },

            Data::AnswerGameData =>  { //ok
                println!("Received: AnswerGameData");
                let answer_gamedata =  recived_packed.data_as_answer_game_data().unwrap();


                return Structures::PacketTypes::AnswerGameData{ motd:answer_gamedata.motd().unwrap_or("").to_string(),
                 low: answer_gamedata.low(), high: answer_gamedata.high(), games:answer_gamedata.games()};
                /*return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};*/
            },
            Data::AnswerPlayer => { //ok
                println!("Received: AnswerPlayer");
                //ReceivePlayer{status: StatusReceivePlayer, player: Player},
                let receive_player = recived_packed.data_as_answer_player().unwrap(); 

                let status = FromPrimitive::from_i32(receive_player.status() as i32).unwrap();

                let player = receive_player.player().unwrap();
                let player_struct = Structures::Player{                    
                    name: player.name().to_string(),
                    auth_token: player.auth_token().unwrap_or("").to_string(),
                    password: player.password().unwrap_or("").to_string(),
                    score: player.score(),
                    is_admin: player.is_admin(),
                    referral: player.referral()
                };

                return Structures::PacketTypes::AnswerPlayer{status: status , player: player_struct};
            },
            Data::AnswerOnlinePlayers =>  { 
                println!("Received: AnswerOnlinePlayers");
                let answer_onlineplayers =   recived_packed.data_as_answer_online_players().unwrap(); 
                let players = answer_onlineplayers.players().unwrap();

                let mut vector_players = Vec::new();
                for i in 0..players.len() {
                    let tmp = players.get(i);
                    vector_players.push(Structures::Player{
                        name: tmp.name().to_string(),
                        auth_token: tmp.auth_token().unwrap_or("").to_string(),
                        password: tmp.password().unwrap_or("").to_string(),
                        score: tmp.score(),
                        is_admin: tmp.is_admin(),
                        referral: tmp.referral()
                    });
                }

                return Structures::PacketTypes::AnswerOnlinePlayers{players: vector_players};
            },
            
            Data::SendGameScore => { //ok
                println!("Received: SendGameScore");
                let gamescore =   recived_packed.data_as_send_game_score().unwrap(); 

                let player = gamescore.player().unwrap();
                let player_struct = Structures::Player{                    
                    name: player.name().to_string(),
                    auth_token: player.auth_token().unwrap_or("").to_string(),
                    password: player.password().unwrap_or("").to_string(),
                    score: player.score(),
                    is_admin: player.is_admin(),
                    referral: player.referral()
                };

                let ms = gamescore.game_result().unwrap();
                let matchscore_struct = Structures::MatchScore {
                    hits: ms.hits(),
                    specials: ms.specials(),
                    misses: ms.misses(),
                    score: ms.score()
                };

                return Structures::PacketTypes::SendGameScore{player: player_struct,
                     game_result: matchscore_struct,
                      score_message: gamescore.score_message().unwrap_or("").to_string()};



               /* return Structures::PacketTypes::AnswerGameData{ motd:answer_gamedata.motd().unwrap_or("").to_string(),
                 low: answer_gamedata.low(), high: answer_gamedata.high(), games:answer_gamedata.games()}; */
            },
            Data::Message => {
                println!("Received: Message");
                let message_serial =   recived_packed.data_as_message().unwrap(); 

                let player = message_serial.from().unwrap();
                let player_struct = Structures::Player{                    
                    name: player.name().to_string(),
                    auth_token: player.auth_token().unwrap_or("").to_string(),
                    password: player.password().unwrap_or("").to_string(),
                    score: player.score(),
                    is_admin: player.is_admin(),
                    referral: player.referral()
                };

                let fcolor = match message_serial.color() {
                    Color::Blue => Structures::Color::Blue,
                    Color::Green => Structures::Color::Green,
                    Color::Red => Structures::Color::Red
                };

                let message = message_serial.text().to_string();


                return Structures::PacketTypes::Message{text: message,  color: fcolor, from: player_struct};
                /*return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};*/
            },
            _ => {
                println!("Received: None");
                return Structures::PacketTypes::None;
            }

        }
    }

    //Packs all data to be sent using a generic packet
    fn pack_data<T>(data: flatbuffers::WIPOffset<T>, mut builder: flatbuffers::FlatBufferBuilder, packet_type: Data)
            -> bytes::Bytes {
        let afp_packet = GenericPacket::create(&mut builder, &GenericPacketArgs{
            data_type: packet_type,
            data: Some(data.as_union_value()),
            ..Default::default()
        });

        builder.finish(afp_packet, None);
        
        bytes::Bytes::from(builder.finished_data().to_vec())        
    }

    fn create_player_data<'a>(player_p: &Structures::Player, mut builder: flatbuffers::FlatBufferBuilder<'a>)
    -> (flatbuffers::FlatBufferBuilder<'a>, flatbuffers::WIPOffset<Player<'a>>)   {
        // alterar tb em online players
        let fplayer_name = builder.create_string(&player_p.name);
        let fplayer_token = builder.create_string(&player_p.auth_token);

        let fplayer = Player::create(&mut builder, &PlayerArgs{
            name: Some(fplayer_name),
            auth_token: Some(fplayer_token),
            referral: player_p.referral,
            ..Default::default()
        });

        (builder, fplayer)
    }


    pub fn ask_for_player(name: &String, password: &String, referral: i64) -> bytes::Bytes{
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let fname = builder.create_string(&name);
        let fpassword = builder.create_string(&password);

        let afp_data = AskForPlayer::create(&mut builder, &AskForPlayerArgs{
            name: Some(fname),
            password: Some(fpassword),
            referral: referral
        });

        pack_data(afp_data, builder, Data::AskForPlayer)
    } 

    pub fn ask_for_game_data(player_p: &Structures::Player) -> bytes::Bytes {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);
        let (mut builder, fplayer) =  create_player_data(&player_p, builder);

        let fask_game_data = AskForGameData::create(&mut builder, &AskForGameDataArgs {
            player: Some(fplayer)
        });

        pack_data(fask_game_data, builder, Data::AskForGameData)
    }


    pub fn ask_for_online_players(sequence_p: u32) -> bytes::Bytes {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let fask_online_players = AskForOnlinePlayers::create(&mut builder, &AskForOnlinePlayersArgs{
            sequence: sequence_p
        });

        pack_data(fask_online_players, builder, Data::AskForOnlinePlayers)
    }
    


    pub fn answer_player(player_p: &Structures::Player, status_p: &Structures::StatusAnswerPlayer) -> bytes::Bytes{
        let builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);
        let (mut builder, fplayer) =  create_player_data(&player_p, builder);

        //StatusAnswerPlayer
        let status = FromPrimitive::from_i32(ToPrimitive::to_i32(status_p).unwrap()).unwrap();


        //needs to modify flat generated to add     
        //    use num_derive::{FromPrimitive, ToPrimitive};    
        //    use num_traits::{FromPrimitive, ToPrimitive};
        //#[derive(FromPrimitive)]
        let fanswer_player = AnswerPlayer::create(&mut builder, &AnswerPlayerArgs{
            player: Some(fplayer),
            status: status
        });

        pack_data(fanswer_player, builder, Data::AnswerPlayer)

    }

    pub fn answer_game_data(motd_p: String, low_p : u32, high_p: u32, games_p: u32) -> bytes::Bytes {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let message = builder.create_string(&motd_p);

        let fgame_data = AnswerGameData::create(&mut builder, &AnswerGameDataArgs{
            motd: Some(message),
            low: low_p,
            high: high_p,
            games: games_p
        });

        pack_data(fgame_data, builder, Data::AnswerGameData)

    }


    pub fn answer_online_players(players_p : &Vec<Structures::Player>) -> bytes::Bytes {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(4096);

        let mut built_players = Vec::new();
        for player in players_p.iter() {

            println!("{:#?}",&player);

            let fplayer_name = builder.create_string(&player.name);
            let fplayer_password = builder.create_string(&player.password);

            let fplayer = Player::create(&mut builder, &PlayerArgs{
                name: Some(fplayer_name),
                password: Some(fplayer_password),
                is_admin: player.is_admin,
                score: player.score,
                ..Default::default()
            });

            built_players.push(fplayer);
        }

        let flat_players = builder.create_vector(&built_players[..]);


        let fanswer_online_players = AnswerOnlinePlayers::create(&mut builder, &AnswerOnlinePlayersArgs{
            players: Some(flat_players)
        });

        pack_data(fanswer_online_players, builder ,Data::AnswerOnlinePlayers)
    }


    pub fn message(player_p: &Structures::Player, message: String, color_p: Structures::Color) -> bytes::Bytes {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let fmessage = builder.create_string(&message);

        let (mut builder, fplayer) =  create_player_data(&player_p, builder);

        let fcolor = match color_p {
            Structures::Color::Blue => Color::Blue,
            Structures::Color::Green => Color::Green,
            Structures::Color::Red => Color::Red
        };

        let fmessage_data = Message::create(&mut builder, &MessageArgs{
            text: Some(fmessage),
            color: fcolor,
            from: Some(fplayer)
        });

        pack_data(fmessage_data, builder, Data::Message)
    }

    pub fn send_game_score(player_p: &Structures::Player, match_score: &Structures::MatchScore, score_message: String)-> bytes::Bytes{
        let builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let (mut builder, fplayer) =  create_player_data(&player_p, builder);


        let fgame_result = GameResult::new(match_score.hits, match_score.specials, match_score.misses, match_score.score);

        let message = builder.create_string(&score_message);

        let score = SendGameScore::create(&mut builder, &SendGameScoreArgs{
            player: Some(fplayer),
            game_result: Some(&fgame_result),
            score_message: Some(message)
        });

        pack_data(score, builder, Data::SendGameScore)

    }

    
    
}