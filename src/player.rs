use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::fs::{read, write};

use crate::fish::Fishes;

#[derive(Deserialize, Serialize, Debug)]
pub struct Player {
    name: String,
    level: u32,
    xp: u32,
    gold: u32,
    inventory: HashMap<String, u32>,
}

impl Player {
    // Creates new Player
    pub fn new(name: String) -> Player {
        Player {
            name,
            level: 1,
            xp: 0,
            gold: 0,
            inventory: HashMap::new(),
        }
    }

    pub fn from_file(filename: &str) -> std::io::Result<Player> {
        let player = serde_json::from_slice(&read(filename)?)?;

        Ok(player)
    }

    pub fn save_file(&self, filename: &str) -> std::io::Result<()> {
        write(filename, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn cast_line(&mut self, fishes: &Fishes) {
        // Getting a  random fish
        let fish = fishes.get_random_fish().unwrap();
        println!("You caught a {}!", fish.fish_name);

        // Placing fish on inventory
        *self.inventory.entry(fish.fish_name).or_insert(0) += 1;

        // Adding xp from fish to player
        self.xp += fish.xp_worth;
        self.level_up();
    }

    // Print player inventory
    pub fn print_inventory(&self) {
        println!("Inventory: ");
        self.inventory
            .iter()
            .for_each(|(k, v)| println!("{0: <10} | {1: <10}", k, v))
    }

    // Check if player can level up, if possible, levels up
    pub fn level_up(&mut self) {
        if self.xp >= 500 {
            self.level += 1;
            self.xp -= 500;
        }
    }
}

// Equivalent to defining __repr__ on python class
// Implements basic display formatting
impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} | Lv. {}, {}/500 XP | {}g\n",
            self.name, self.level, self.xp, self.gold
        )
    }
}
