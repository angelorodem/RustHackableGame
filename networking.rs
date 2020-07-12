pub mod GameNetworking {

    pub use crate::structures::Structures;
    pub use crate::GenericPacket_generated::{GenericPacket, GenericPacketArgs,Data, get_root_as_generic_packet};
    pub use crate::AskForPlayer_generated::{AskForPlayer,AskForPlayerArgs, get_root_as_ask_for_player};
    pub use crate::Message_generated::{Message, MessageArgs,Color, get_root_as_message};
    pub use crate::Player_generated::{Player, PlayerArgs, get_root_as_player};
    pub use crate::SendPlayerGameScore_generated::{SendGameScore, SendGameScoreArgs, get_root_as_send_game_score};
    pub use crate::GameResult_generated::{GameResult, GameResultArgs, get_root_as_game_result};


    fn send_to_server(data : &[u8]){

        
        println!("{:?}",&data);
    }

    fn receive_packet(data : &[u8]){
        //let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);
        //let buf = builder.finished_data();         
        let recived_packed = get_root_as_generic_packet(data);

        //let afp_data_re = afp_packet_re.data_as_ask_for_player().unwrap();
        
        //AskForPlayer, GameData, Message, OnlinePlayers, ReceivePlayer, SendGameScore
        match recived_packed.data_type() {
            Data::AskForPlayer => {
                println!("AskForPlayer");
            },
            Data::GameData => {
                println!("GameData");
            },
            Data::Message => {
                println!("Message");
            },
            Data::OnlinePlayers =>  {
                println!("OnlinePlayers");
            },
            Data::ReceivePlayer => {
                println!("ReceivePlayer");
            },
            Data::SendGameScore => {
                println!("SendGameScore");
            },
            _ => {
                println!("none of above");
            }

        }
        
        
        
        //println!("{:?}",);

    }

    fn send_packet<T>(data: flatbuffers::WIPOffset<T>, mut builder: flatbuffers::FlatBufferBuilder, packet_type: Data){
        let afp_packet = GenericPacket::create(&mut builder, &GenericPacketArgs{
            data_type: packet_type,
            data: Some(data.as_union_value()),
            ..Default::default()
        });

        builder.finish(afp_packet, None);

        send_to_server(builder.finished_data());
    }

    pub fn ask_for_player(name: &String, password: &String){
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let fname = builder.create_string(&name);
        let fpassword = builder.create_string(&password);

        let afp_data = AskForPlayer::create(&mut builder, &AskForPlayerArgs{
            name: Some(fname),
            password: Some(fpassword)
        });

        send_packet(afp_data, builder, Data::AskForPlayer);
    }

    fn player_packet<'a>(player_p: &Structures::Player, mut builder: flatbuffers::FlatBufferBuilder<'a>)
                    -> (flatbuffers::FlatBufferBuilder<'a>, flatbuffers::WIPOffset<Player<'a>>)
    {

        let fplayer_name = builder.create_string(&player_p.name);
        let fplayer_token = builder.create_string(&player_p.token);

        let fplayer = Player::create(&mut builder, &PlayerArgs{
            name: Some(fplayer_name),
            auth_token: Some(fplayer_token),
            ..Default::default()
        });

        (builder, fplayer)
    }

    pub fn send_message(player_p: &Structures::Player, message: String, color_p: Structures::Color){
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let fmessage = builder.create_string(&message);

        let (mut builder, fplayer) =  player_packet(&player_p, builder);

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

        send_packet(fmessage_data, builder, Data::Message);
    }

    pub fn send_score(player_p: &Structures::Player, score_message: String){
        let builder = flatbuffers::FlatBufferBuilder::new_with_capacity(512);

        let (mut builder, fplayer) =  player_packet(&player_p, builder);


        let fgame_result = GameResult::create(&mut builder, &GameResultArgs{
            hits: player_p.game_score.hits,
            misses: player_p.game_score.misses,
            specials: player_p.game_score.specials,
            score: player_p.game_score.score,
        });

        let message = builder.create_string(&score_message);

        let game_score = SendGameScore::create(&mut builder, &SendGameScoreArgs{
            player: Some(fplayer),
            game_result: Some(fgame_result),
            score_message: Some(message)
        });

        send_packet(game_score, builder, Data::SendGameScore);

    }
}