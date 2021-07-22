use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, BorshStorageKey};
use near_sdk::collections::LookupMap;

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize)]
pub enum HeroClass {
    Orc,
    Mage,
    Cleric,
    NEARkat
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct PlayerV1 {
    pub name: String,
    pub hero_class: HeroClass,
    pub health: u8,
    pub level: u8,
}

// We know we'll want to update this Player struct as the game grows
#[derive(BorshDeserialize, BorshSerialize)]
pub enum Player {
    CurrentVersion(PlayerV1)
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    players: Vec<Player>,
    winner: Option<Player> // None if the game is ongoing
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    games: LookupMap<u32, Game>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    GameKey,
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            games: LookupMap::new(StorageKey::GameKey)
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn fight(&mut self) {
        env::log(b"Fighting stub!");
    }
}
