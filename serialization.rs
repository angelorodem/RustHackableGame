#[allow(non_snake_case, dead_code, unused_imports)]
pub mod Serialization {

    use bytes::{Bytes};
    use internet_checksum::checksum;

    pub use crate::structures::Structures;

    use num_derive::{FromPrimitive, ToPrimitive};    
    use num_traits::{FromPrimitive, ToPrimitive};

    pub use crate::GenericPacket_generated::{GenericPacket, GenericPacketArgs,Data, get_root_as_generic_packet};
    pub use crate::AskForPlayer_generated::{AskForPlayer,AskForPlayerArgs, get_root_as_ask_for_player};
    pub use crate::Message_generated::{Message, MessageArgs,Color, get_root_as_message};
    pub use crate::Player_generated::{Player, PlayerArgs, get_root_as_player};
    pub use crate::SendPlayerGameScore_generated::{SendGameScore, SendGameScoreArgs, get_root_as_send_game_score};
    pub use crate::GameResult_generated::{GameResult, GameResultArgs, get_root_as_game_result};
    pub use crate::AnswerPlayer_generated::{AnswerPlayer, AnswerPlayerArgs, get_root_as_answer_player, StatusAnswerPlayer};
    pub use crate::GameData_generated::{GameData, GameDataArgs, get_root_as_game_data};


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
        //AskForPlayer, GameData, Message, OnlinePlayers, ReceivePlayer, SendGameScore
        match recived_packed.data_type() {
            Data::AskForPlayer => {
                println!("Received: AskForPlayer");   
                let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap();  
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};
            },
            Data::GameData => {
                println!("Received: GameData");
                let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap(); 
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};
            },
            Data::Message => {
                println!("Received: Message");
                let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap(); 
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};
            },
            Data::OnlinePlayers =>  {
                println!("Received: OnlinePlayers");
                let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap(); 
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};
            },
            Data::AnswerPlayer => {
                println!("Received: ReceivePlayer");
                //ReceivePlayer{status: StatusReceivePlayer, player: Player},
                let receive_player = recived_packed.data_as_answer_player().unwrap(); 

                let status = FromPrimitive::from_i32(receive_player.status() as i32).unwrap();
                let player = receive_player.player().unwrap();

                let player_struct = Structures::Player{                    
                    name: player.name().to_string(),
                    auth_token: player.auth_token().unwrap_or("").to_string(),
                    password: player.password().unwrap_or("").to_string(),
                    score: player.score(),
                    is_admin: player.is_admin()
                };

                return Structures::PacketTypes::AnswerPlayer{status: status , player: player_struct};
            },
            Data::SendGameScore => {
                println!("Received: SendGameScore");
                let ask_for_player =   recived_packed.data_as_ask_for_player().unwrap(); 
                return Structures::PacketTypes::AskForPlayer{name: ask_for_player.name().unwrap().to_string()
                    , password: ask_for_player.password().unwrap().to_string()};
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

    pub fn ask_for_player(name: &String, password: &String) -> bytes::Bytes{
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let fname = builder.create_string(&name);
        let fpassword = builder.create_string(&password);

        let afp_data = AskForPlayer::create(&mut builder, &AskForPlayerArgs{
            name: Some(fname),
            password: Some(fpassword)
        });

        pack_data(afp_data, builder, Data::AskForPlayer)
    }

    fn create_player_data<'a>(player_p: &Structures::Player, mut builder: flatbuffers::FlatBufferBuilder<'a>)
                    -> (flatbuffers::FlatBufferBuilder<'a>, flatbuffers::WIPOffset<Player<'a>>)
    {

        let fplayer_name = builder.create_string(&player_p.name);
        let fplayer_token = builder.create_string(&player_p.auth_token);

        let fplayer = Player::create(&mut builder, &PlayerArgs{
            name: Some(fplayer_name),
            auth_token: Some(fplayer_token),
            ..Default::default()
        });

        (builder, fplayer)
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


        let fgame_result = GameResult::create(&mut builder, &GameResultArgs{
            hits: match_score.hits,
            misses: match_score.misses,
            specials: match_score.specials,
            score: match_score.score,
        });

        let message = builder.create_string(&score_message);

        let score = SendGameScore::create(&mut builder, &SendGameScoreArgs{
            player: Some(fplayer),
            game_result: Some(fgame_result),
            score_message: Some(message)
        });

        pack_data(score, builder, Data::SendGameScore)

    }

    pub fn answer_player(player_p: &Structures::Player, status_p: &Structures::StatusAnswerPlayer) -> bytes::Bytes{
        let builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);
        let (mut builder, fplayer) =  create_player_data(&player_p, builder);

        //StatusAnswerPlayer
        let status = FromPrimitive::from_i32(ToPrimitive::to_i32(status_p).unwrap()).unwrap();


        //needs to modify flat generated to add     #[derive(FromPrimitive)]
        let fanswer_player = AnswerPlayer::create(&mut builder, &AnswerPlayerArgs{
            player: Some(fplayer),
            status: status
        });

        pack_data(fanswer_player, builder, Data::AnswerPlayer)


    }

    pub fn game_data(motd: String, low : u32, high: u32, games: u32) -> bytes::Bytes {
        let builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let message = builder.create_string(&motd);

        let fgame_data = GameData::create(&mut builder, GameDataArgs{
            motd: message,
            low;
            high;
            games;    
        });

        pack_data(fgame_data, builder, Data::GameData)

    }


    
    
}