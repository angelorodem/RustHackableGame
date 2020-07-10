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

use colored::*;
use rand::Rng;
use std::io;

#[derive(Debug)]
struct MatchScore {
    hits: i32,
    specials: i32,
    misses: i32,
    score: i32
}

//This need better DOP
#[derive(Debug)]
struct Player {
    name: String,
    nick: String,
    total_score: i32,
    game_scores: Vec<MatchScore>
}

enum GamePackets<'a> {
    AskForPlayer(String,String),
    SendMessage(&'a Player, String),
    SendScore(&'a Player)
}

fn get_name() -> (String, String) {
    let mut name: String = String::new();

    println!("{}","Please insert your name!".yellow());
    
    io::stdin().read_line(&mut name).expect("Expected string.");
    let name = name.trim();

    if name.len() < 3 {
        panic!("{}{}","I bet your name is not this short".red(),", try again".green().bold());
    }

    let nick = String::from(&name[..3]);

    (name.to_string(), nick)
}


fn get_guesses(low: &i32, high: &i32) -> Result<Vec<i32>,()> {
    
    let mut str_input = String::new();

    io::stdin().read_line(&mut str_input).expect("Expected string");

    let str_numbers : Vec<&str> = str_input.trim().split(" ").collect();

    if str_numbers.len() != 5 { 
        println!("{}{}","Oh no, i hope you can count to 5, like... just use your fingers bro".red(),", try again".green().bold());
        return Err(());
    }


    let mut guesses : Vec<i32> = Vec::new();

    for i in str_numbers.iter(){
        let n = i.parse();
        if let Err(_) = n {
            println!("Please insert only numbers");
            return Err(());
        }
        let n = n.unwrap();

        if n < *low || n >= *high {
            println!("{}{}",format!("Oh no no, the number should be bigger than {} and lower than {}",low,high).red().bold(),
            ", try again".green().bold());
            return Err(());
        }
        guesses.push(n);
    }

    Ok(guesses)
}


fn check_guesses(guesses : &Vec<i32>, player : &mut Player, low: &i32, high: &i32){
    let mut rng = rand::thread_rng();
    
    let mut match_score = MatchScore {
        hits : 0,
        specials: 0,
        misses: 0,
        score: 0,
    };

    let mut sum = 0;

    for (i, &val) in guesses.iter().enumerate() {
        let rand = rng.gen_range(low,high);
        print!("{}", format!("Peek: G: {} P: {} ", val,rand).purple());
        if val ==  rand {
            println!("{} {} {}", "Guess".green(),i,"Hit!".blue().bold());
            match_score.hits += 1;
            sum += 11;
        } else if (val > 1)  && (rand % val == 0) {
            println!("{} {} {}", "Guess".green(),i,"Missed in a special way!".yellow().bold());
            match_score.specials += 1;
            sum += 2;
        } else {
            println!("{} {} {}", "Guess".green(),i,"Missed!".red().bold());
            match_score.misses += 1;
            sum -= 3;
        }   
    }

    match_score.score = if sum % 2 == 0 { sum } else { 
        println!("Extra point for score being ODD");
        sum +1 
    };

    player.total_score += match_score.score;
    player.game_scores.push(match_score);
    
}




fn main() {  
    println!("Welcome to the Online {} Gambling game!", "Hackable".black());
    
    let (name, nick) = get_name();
    let games = 3;
    let low = 1;
    let high = 7;
    

    println!("\n\n {}, {} ({})","Hello".green(),name.red(),nick.red().bold());
    println!("{} {} {}","We will play".yellow() ,games,"rounds of Guess the number! (With special numbers)".yellow());
    println!("For each round, you have to guess 5 random numbers from {} in sequence,
     \ntry guessing by inputting 5 numbers separated by space!", format!("{}-{}",low,high-1).red().bold());

    let mut player = Player {
        name,
        nick,
        total_score : 0,
        game_scores : Vec::new()        
    };
    
    let mut count : i32 = 0;

    while count < games {

        println!("{} {}","Ok, fire your 5 guesses for game: ".yellow(),count);
        let guesses = get_guesses(&low,&high);

        if let Err(_) = guesses {
            continue;
        }

        println!("You entered: {:?}", &guesses);
        check_guesses(&guesses.unwrap(), &mut player, &low, &high);

        count += 1;
    }

    
    player.game_scores.iter().enumerate().for_each(|(i,x)| {
        println!("Round: {} | Hits: {} Espc: {} Miss: {} -- Round Score: {}",i,&x.hits,&x.specials,&x.misses,&x.score);
    });

    println!("Games score!: {}", &player.total_score);




}
