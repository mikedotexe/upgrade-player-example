use crate::hero_class::AbilityEffects;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum Item {
    Shield,
    Sword,
    HealingPotion,
}

pub fn get_item_abilities(item: &Item) -> Vec<AbilityEffects> {
    match item {
        Item::Shield => vec![AbilityEffects {
            self_damage_reduction: Some(15),
            ..Default::default()
        }],
        Item::Sword => vec![AbilityEffects {
            target_damage: Some(15),
            ..Default::default()
        }],
        Item::HealingPotion => vec![AbilityEffects {
            self_heal: Some(50),
            ..Default::default()
        }],
    }
}
