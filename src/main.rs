mod fish;
mod player;
mod utils;

use crate::fish::Fishes;
use crate::player::Player;
use crate::utils::input;

const FISH_FILE: &str = "fishDB.json";
const PLAYER_FILE: &str = "playerDB.json";

fn main() {
    let fishes = match Fishes::from_file(FISH_FILE) {
        Ok(fishes) => fishes,
        Err(error) => panic!("Error opening fish data: {}", error.to_string()),
    };

    // If there is no save file, create a new player
    let mut p = match Player::from_file(PLAYER_FILE) {
        Ok(p) => p,
        Err(_) => {
            let p_name = input("Ahoy fisherman! I see you're new here! What's your name?\n>>> ");
            // Stripping \n then creating player from input
            Player::new(p_name.strip_suffix("\n").unwrap().to_string())
        }
    };

    // Game loop
    loop {
        let p_action = input("Options: [f]ish | [i]nventory | [p]layer info | [e]xit\n>>> ");

        match p_action.chars().next().unwrap() {
            'f' => p.cast_line(&fishes),
            'p' => print!("{}", p),
            'i' => p.print_inventory(),
            'e' => break,
            _ => println!("Invalid Command!"),
        }
    }

    if let Err(error) = p.save_file(PLAYER_FILE) {
        println!("Error saving player data: {}", error.to_string())
    }
    if let Err(error) = fishes.save_file(FISH_FILE) {
        println!("Error saving fishes: {}", error.to_string())
    }
    println!("Thanks for playing!");
}
