use crate::hero_class::{
    ability_variant_eq, get_ability_variant, get_base_stats, get_hero_abilities, AbilityEffects,
    HeroAbility, HeroClass,
};
use crate::items::{get_item_abilities, Item};
use crate::log_events::{LogEvent, LogLevel};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, TreeMap};
use near_sdk::log;
use near_sdk::{
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey,
};

mod hero_class;
mod items;
pub mod log_events;

near_sdk::setup_alloc!();

pub const CRITICAL_STRIKE: u8 = 5;

type Lvl = LogLevel;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct PlayerV1 {
    pub name: String,
    pub hero_class: HeroClass,
    pub health: u8,
    pub level: u8,
    pub items: Vec<Item>,
}

// #[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct PlayerV2 {
//     pub name: String,
//     pub hero_class: HeroClass,
//     pub health: u8,
//     pub level: u8,
//     pub mana: u8, // new field
// }

// We know we'll want to update this Player struct as the game grows
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Player {
    BaseVersion(PlayerV1),
    // VersionWithMana(PlayerV2)
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Game {
    players: TreeMap<AccountId, Player>,
    winner: Option<Player>, // None if the game is ongoing
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
    Player(u32),
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            games: LookupMap::new(StorageKey::Game),
            games_active: false,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn internal_demo(&mut self) {
        env::panic(&LogEvent::new(Lvl::ContractError, "NO_USER_EXISTS").b())
    }
    pub fn internal_demo2(&mut self) {
        let hi: Option<String> = None;
        hi.expect(r#"{"level":"ContractError","code":"LOGEVENT_TO_STRING"}"#);
    }

    pub fn use_ability(&mut self, ability: String, target: AccountId, game_num: u32) {
        let ability_obj = get_ability_variant(&ability).expect(
            &LogEvent::new(Lvl::UserError, "NO_ABILITY")
                .advice(format!(
                    "Please use one of these abilities: {}.",
                    self.get_abilities()
                ))
                .to_string(),
        );
        let mut game = self.games.get(&game_num).expect("Game doesn't exist.");
        let player = game.players.get(&env::predecessor_account_id()).expect("Not registered as a player for this game, please add yourself first by calling add_player.");
        let target_player = game
            .players
            .get(&target)
            .expect("Target account not registered as a player for this game.");

        let mut player_obj = self.get_player_obj(&player);
        let mut target_obj = self.get_player_obj(&target_player);

        let hero_class = &player_obj.hero_class;
        let class_abilities = get_hero_abilities(&hero_class);

        let player_ability: Vec<&HeroAbility> = class_abilities
            .iter()
            .filter(|&p| ability_variant_eq(p, &ability_obj))
            .collect();

        assert_ne!(
            player_ability.len(),
            0,
            "Your player doesn't have this ability."
        );
        log!("Using player_ability {:?}", player_ability);
        let player_ability_effects = match player_ability[0] {
            HeroAbility::Strike(a) => a,
            HeroAbility::HeadBonk(a) => a,
            HeroAbility::Heal(a) => a,
            HeroAbility::Ultimate(a) => a,
        };

        let (actor_effect, target_effect) =
            self.sum_effects(&player, player_ability_effects, &target_player);
        log!(
            "Your player {}s, resulting in:\
            \nActing player: {:?}\
            \nTarget player: {:?}",
            ability,
            actor_effect,
            target_effect
        );
        // let combined_effects = actor_effect + target_effect;
        // log!("Combined effects: {:?}", combined_effects);

        // Subtract health from acting player
        let damage_to_self = actor_effect.self_damage.unwrap_or_else(|| 0);
        player_obj.health -= damage_to_self;
        player_obj.level += 1; // Player's level increases when they use abilities

        // Subtract health from targeted player
        let mut damage_to_target = actor_effect.target_damage.unwrap_or_else(|| 0);
        log!("aloha damage_to_target {}", damage_to_target);
        // If the acting player just hit a level divisible by 6, add critical strike damage
        if player_obj.level % 6 == 0 {
            damage_to_target += CRITICAL_STRIKE * (player_obj.level / 6);
        }
        target_obj.health -= damage_to_target;

        log!("Acting player is at {} health", player_obj.health);
        log!("Targeted player is at {} health", target_obj.health);

        // See if anyone is below or at zero health and if they have healing potions
        // They lose all items
        // Set up closure
        let mut check_death_or_heal =
            |mut player: PlayerV1, account, ability_effects: AbilityEffects| {
                if player.health <= 0 {
                    let heal_to_self = ability_effects.self_heal.unwrap_or_else(|| 0);
                    player.health += heal_to_self;

                    // If they're still at or below zero, remove them from the game
                    if player.health <= 0 {
                        game.players.remove(&account);
                        self.games.insert(&game_num, &game);
                        log!("Player {} died!", player.name);
                    } else {
                        // Insert "over" (AKA edit) the entry for this player
                        game.players.insert(&account, &Player::BaseVersion(player));
                    }
                } else {
                    game.players.insert(&account, &Player::BaseVersion(player));
                }
            };

        check_death_or_heal(player_obj, env::predecessor_account_id(), actor_effect);
        check_death_or_heal(target_obj, target, target_effect);
        //     for mut player in [player_obj, target_obj] {
        //         if player.health <= 0 {
        //             let heal_to_self = combined_effects.self_heal.unwrap_or_else(|| 0);
        //             // We'll assume there is no negative healing
        //             player.health += heal_to_self.abs() as u8;
        //
        //             // If they're still at or below zero, remove them from the game
        //             if player.health <= 0 {
        //                 game.players.remove(&player.account);
        //                 self.games.insert(&game_num, &game);
        //                 log!("Player {} died!", player.name);
        //             } else {
        //                 // Insert "over" (AKA edit) the entry for this player
        //                 game.players.insert(&player.account.clone(), &Player::BaseVersion(player));
        //             }
        //         } else {
        //             game.players.insert(&player.account.clone(), &Player::BaseVersion(player));
        //         }
        //     }
    }

    pub fn add_player(&mut self, hero_class: HeroClass, name: String, game_num: u32) {
        let (health, level) = get_base_stats(&hero_class);
        if name.len() > 16 {
            env::panic(b"Name must be at most 16 characters");
        }
        let player = PlayerV1 {
            name,
            hero_class,
            health,
            level,
            items: vec![],
        };
        let p = Player::BaseVersion(player);
        let mut game = self.games.get(&game_num).unwrap_or_else(|| Game {
            players: TreeMap::new(StorageKey::Player(game_num)),
            winner: None,
        });
        game.players.insert(&env::predecessor_account_id(), &p);
        self.games.insert(&game_num, &game);
    }

    pub fn get_game_players(&self, game_num: u32) -> Vec<(AccountId, Player)> {
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

    // TODO: potentially move this into hero_class.rs
    pub fn get_abilities(&self) -> String {
        let ability_vec: Vec<String> = HeroAbility::VALUES.iter().map(|s| s.to_string()).collect();
        if let Some((last, els)) = ability_vec.split_last() {
            els.join(", ") + ", or " + last
        } else {
            env::panic(&LogEvent::new(Lvl::ContractError, "INVALID_ABILITIES").b())
        }
    }

    /// Returns a tuple of the ability effects for the actor and target
    fn sum_effects(
        &self,
        actor: &Player,
        ability_effects: &AbilityEffects,
        target: &Player,
    ) -> (AbilityEffects, AbilityEffects) {
        // Start actor default with whatever this ability offers or 0 (for easy algebra)
        // The actor's items may alter this
        let mut final_effects_actor = AbilityEffects {
            target_damage: Some(ability_effects.target_damage.unwrap_or_else(|| 0)),
            self_damage: Some(ability_effects.self_damage.unwrap_or_else(|| 0)),
            target_heal: Some(ability_effects.target_heal.unwrap_or_else(|| 0)),
            self_heal: Some(ability_effects.self_heal.unwrap_or_else(|| 0)),
            self_damage_reduction: Some(ability_effects.self_damage_reduction.unwrap_or_else(|| 0)),
        };
        // The target starts at 0 for all effects, their items may alter this
        let mut final_effects_target = AbilityEffects {
            target_damage: Some(0),
            self_damage: Some(0),
            target_heal: Some(0),
            self_heal: Some(0),
            self_damage_reduction: Some(0),
        };

        let actor_items = match &actor {
            Player::BaseVersion(p) => &p.items,
        };
        // Check if actor and target are the same, like healing one's self
        if actor == target {
            env::panic(b"Cannot do stuff to yourself at the moment.")
        } else {
            // Targeting another player
            for item in actor_items.iter() {
                // Loop through the effects and add (or subtract) them accordingly
                for ability in get_item_abilities(item).iter() {
                    final_effects_actor.target_damage = Some(
                        final_effects_actor.target_damage.unwrap()
                            + ability.target_damage.unwrap_or_else(|| 0),
                    );
                    final_effects_actor.self_damage = Some(
                        final_effects_actor.target_damage.unwrap()
                            + ability.self_damage.unwrap_or_else(|| 0),
                    );
                    final_effects_actor.target_heal = Some(
                        final_effects_actor.target_damage.unwrap()
                            + ability.target_heal.unwrap_or_else(|| 0),
                    );
                    final_effects_actor.self_heal = Some(
                        final_effects_actor.target_damage.unwrap()
                            + ability.self_heal.unwrap_or_else(|| 0),
                    );
                }
            }

            let target_items = match &target {
                Player::BaseVersion(p) => &p.items,
            };
            for item in target_items.iter() {
                // Loop through the effects and add (or subtract) them accordingly
                for ability in get_item_abilities(item).iter() {
                    final_effects_target.target_damage = Some(
                        final_effects_target.target_damage.unwrap()
                            + ability.target_damage.unwrap_or_else(|| 0),
                    );
                    final_effects_target.self_damage = Some(
                        final_effects_target.target_damage.unwrap()
                            + ability.self_damage.unwrap_or_else(|| 0),
                    );
                    final_effects_target.target_heal = Some(
                        final_effects_target.target_damage.unwrap()
                            + ability.target_heal.unwrap_or_else(|| 0),
                    );
                    final_effects_target.self_heal = Some(
                        final_effects_target.target_damage.unwrap()
                            + ability.self_heal.unwrap_or_else(|| 0),
                    );
                }
            }
        }
        (final_effects_actor, final_effects_target)
    }

    fn get_player_obj(&self, player: &Player) -> PlayerV1 {
        match player {
            Player::BaseVersion(p) => PlayerV1 {
                name: p.name.clone(),
                hero_class: p.hero_class.clone(),
                health: p.health.clone(),
                level: p.level.clone(),
                items: p.items.clone(),
            },
        }
    }
}
