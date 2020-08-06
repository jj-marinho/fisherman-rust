use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{read, write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fish {
    pub fish_name: String,
    pub chance: u32,
    pub xp_worth: u32,
    pub gold_worth: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fishes(Vec<Fish>);

impl Clone for Fish {
    fn clone(&self) -> Self {
        Fish {
            fish_name: self.fish_name.clone(),
            chance: self.chance,
            xp_worth: self.xp_worth,
            gold_worth: self.gold_worth,
        }
    }
}

impl IntoIterator for Fishes {
    type Item = Fish;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Fishes {
    pub fn from_file(filename: &str) -> Result<Fishes, Box<dyn std::error::Error>> {
        let fishes = serde_json::from_slice(&read(filename)?)?;
        Ok(fishes)
    }

    pub fn save_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        write(filename, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    // Given a Vec<Fish> and each fish's chance of being captured, gets a random fish
    pub fn get_random_fish(&self) -> Option<Fish> {
        // Getting sum of chances
        let sum_chances = self
            .clone()
            .into_iter()
            .fold(0, |sum, fish| sum + fish.chance);

        // Getting random number that is between 0 and sum_chances
        let rng = rand::thread_rng().gen_range(0, sum_chances - 1);

        // Given the rng result, returns Some(Fish)
        let mut cur_chance = 0;
        for fish in self.clone().into_iter() {
            cur_chance += fish.chance;

            if rng <= cur_chance {
                return Some(fish.clone());
            }
        }
        return None;
    }
}
