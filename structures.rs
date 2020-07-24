pub mod Structures {
    
    use bytes::Bytes;
    use std::sync::{Mutex};
    use num_derive::{FromPrimitive, ToPrimitive};    
    use num_traits::{FromPrimitive, ToPrimitive};

    #[derive(Debug)]
    #[derive(Default)]
    pub struct MatchScore {
        pub hits: u32,
        pub specials: u32,
        pub misses: u32,
        pub score: i32
    }

    #[derive(Debug)]
    #[derive(Default)]
    pub struct Player {
        pub name: String,
        pub auth_token: String,
        pub password: String,
        pub score: u64,
        pub is_admin: bool
    }

    #[derive(Debug)]
    #[derive(FromPrimitive)]
    pub enum Color  {
        Red = 0,
        Green = 1,
        Blue = 2
    }

    #[derive(Debug)]
    #[derive(FromPrimitive, ToPrimitive)]
    pub enum StatusAnswerPlayer {
        OkNew = 0,
        OkLogin = 1,
        Denied = 2,
        Failure = 3
    }

    #[derive(Debug)]
    pub enum PacketTypes {
        AskForPlayer{name: String, password: String},
        GameData{ motd: String, low : u32, high: u32, games: u32},
        Message{text: String,  color: Color, from: Player},
        OnlinePlayers{players: Vec<Player>},
        AnswerPlayer{status: StatusAnswerPlayer, player: Player},
        SendGameScore{player: Player, game_result: MatchScore, score_message: String},
        None
    }

    pub type Packets = Vec<bytes::Bytes>;

}