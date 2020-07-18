pub mod Structures {
    use std::sync::{Mutex};

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
        pub token: String,
        pub game_score: MatchScore
    }

    pub enum Color  {
        Red = 0,
        Green = 1,
        Blue = 2
    }

    pub struct SharedVector {
        pub svec: Mutex<Vec<String>>
    }
}