pub mod Structures {
    #[derive(Debug)]
    pub struct MatchScore {
        pub hits: i32,
        pub specials: i32,
        pub misses: i32,
        pub score: i32
    }

    #[derive(Debug)]
    pub struct Player {
        pub name: String,
        pub game_scores: Vec<MatchScore>
    }
}