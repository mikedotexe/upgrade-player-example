use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, BorshStorageKey, serde::{Deserialize, Serialize}};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::log;

near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum HeroClass {
    Orc,
    Mage,
    Cleric,
    NEARkat
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayerV1 {
    pub name: String,
    pub hero_class: HeroClass,
    pub health: u8,
    pub level: u8,
}

// We know we'll want to update this Player struct as the game grows
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Player {
    CurrentVersion(PlayerV1)
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    players: UnorderedSet<Player>,
    winner: Option<Player> // None if the game is ongoing
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    games: LookupMap<u32, Game>,
    games_active: bool,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Game,
    Player
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            games: LookupMap::new(StorageKey::Game),
            games_active: false
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn fight(&mut self) {
        env::log(b"Fighting stub!");
    }

    pub fn add_player(&mut self, game_num: u32, player: PlayerV1) {
        let p = Player::CurrentVersion(player);
        let mut game = self.games.get(&game_num).unwrap_or_else(||
            Game {
                players: UnorderedSet::new(StorageKey::Player),
                winner: None
            }
        );
        game.players.insert(&p);
        self.games.insert(&game_num, &game);
    }

    pub fn get_game_players(&self, game_num: u32) -> Vec<Player> {
        let game = self.games.get(&game_num).expect("Couldn't find game.");
        game.players.to_vec()
    }

    pub fn start_game(&mut self, game_num: u32) {
        log!("Starting game {}", game_num);
        // Stub
        self.games_active = true;
    }

    pub fn stop_game(&mut self, game_num: u32) {
        log!("Stopping game {}", game_num);
        // Stub
        self.games_active = false;
    }
}
