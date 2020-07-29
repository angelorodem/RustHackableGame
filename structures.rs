pub mod Structures {
    
    use bytes::Bytes;
    use tokio::sync::Mutex;
    use num_derive::{FromPrimitive, ToPrimitive};    
    use num_traits::{FromPrimitive, ToPrimitive};

    #[derive(Debug)]
    #[derive(Default)]
    pub struct MatchScore {
        pub hits: u32,
        pub specials: u32,
        pub misses: u32,
        pub score: i64
    }

    #[derive(Debug,Default,Clone)]
    pub struct Player {
        pub name: String,
        pub auth_token: String,
        pub password: String,
        pub score: i64,
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
        AskForGameData{player: Player},
        AskForOnlinePlayers{sequence_p: u32},

        AnswerPlayer{status: StatusAnswerPlayer, player: Player},
        AnswerOnlinePlayers{players: Vec<Player>},
        AnswerGameData{ motd: String, low : u32, high: u32, games: u32},

        Message{text: String,  color: Color, from: Player},        
        SendGameScore{player: Player, game_result: MatchScore, score_message: String},
        None
    }

    pub type Packets = Vec<bytes::Bytes>;

    pub struct IncomingPackets {
        pub data: Mutex<Packets>
    }

    //Diffrent structs to make more strict wich struct get accessed at certain moment
    pub struct OutgoingPackets {
        pub data: Mutex<Packets>
    }

}