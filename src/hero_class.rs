use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::export::fmt;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum HeroClass {
    Ogre,
    Mage,
    Cleric,
}

/// returns tuple with health and starting level
pub(crate) fn get_base_stats(class: &HeroClass) -> (u8, u8) {
    match class {
        HeroClass::Ogre => (150, 1),
        HeroClass::Mage => (100, 3),
        HeroClass::Cleric => (80, 5),
    }
}

#[derive(Deserialize, Default, PartialEq, Copy, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AbilityEffects {
    pub target_damage: Option<u8>,
    pub self_damage: Option<u8>,
    pub target_heal: Option<u8>,
    pub self_heal: Option<u8>,
    pub self_damage_reduction: Option<u8>,
}

/// Gives the ability to add effects together
/// See: https://doc.rust-lang.org/std/ops/trait.Add.html#examples
/// Might not need this, but we can add if needed
impl std::ops::Add for AbilityEffects {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            target_damage: Some(
                self.target_damage.unwrap_or_else(|| 0) + other.target_damage.unwrap_or_else(|| 0),
            ),
            self_damage: Some(
                self.self_damage.unwrap_or_else(|| 0) + other.self_damage.unwrap_or_else(|| 0),
            ),
            target_heal: Some(
                self.target_heal.unwrap_or_else(|| 0) + other.target_heal.unwrap_or_else(|| 0),
            ),
            self_heal: Some(
                self.self_heal.unwrap_or_else(|| 0) + other.self_heal.unwrap_or_else(|| 0),
            ),
            self_damage_reduction: Some(
                self.self_damage_reduction.unwrap_or_else(|| 0)
                    + other.self_damage_reduction.unwrap_or_else(|| 0),
            ),
        }
    }
}

// TODO: possibly use Result instead of Option
pub(crate) fn get_ability_variant(ability_plain: &String) -> Option<HeroAbility> {
    match ability_plain.as_str() {
        "strike" => Some(HeroAbility::Strike(Default::default())),
        "headbonk" | "bonk" => Some(HeroAbility::HeadBonk(Default::default())),
        "heal" => Some(HeroAbility::Heal(Default::default())),
        "ultimate" | "ult" => Some(HeroAbility::Ultimate(Default::default())),
        _ => None,
    }
}

#[derive(Deserialize, PartialEq, Copy, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum HeroAbility {
    Strike(AbilityEffects),
    HeadBonk(AbilityEffects),
    Heal(AbilityEffects),
    Ultimate(AbilityEffects),
}

impl HeroAbility {
    pub(crate) const VALUES: [&'static str; 4] = ["strike", "headbonk", "heal", "ultimate"];
}

/// Thank you Shepmaster (as always) https://stackoverflow.com/a/32554326/711863
pub(crate) fn ability_variant_eq(a: &HeroAbility, b: &HeroAbility) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

// impl FromIterator<HeroAbility> for HeroAbility {
//     fn from_iter<I: IntoIterator<Item=HeroAbility>>(iter: I) -> Self {
//         let mut c = HeroAbility::default();
//
//         for i in iter {
//             c.add(i);
//         }
//
//         c
//     }
// }

impl fmt::Display for HeroAbility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub fn get_hero_abilities(class: &HeroClass) -> Vec<HeroAbility> {
    match class {
        HeroClass::Ogre => vec![
            HeroAbility::Strike(AbilityEffects {
                target_damage: Some(20),
                ..Default::default()
            }),
            HeroAbility::HeadBonk(AbilityEffects {
                target_damage: Some(25),
                self_damage: Some(5),
                ..Default::default()
            }),
            HeroAbility::Ultimate(AbilityEffects {
                target_damage: Some(30),
                ..Default::default()
            }),
        ],
        HeroClass::Mage => vec![
            HeroAbility::Strike(AbilityEffects {
                target_damage: Some(25),
                self_damage: Some(5),
                target_heal: None,
                self_heal: None,
                self_damage_reduction: None,
            }),
            HeroAbility::Heal(AbilityEffects {
                target_damage: Some(25),
                self_damage: Some(5),
                target_heal: None,
                self_heal: None,
                self_damage_reduction: None,
            }),
            HeroAbility::Ultimate(AbilityEffects {
                target_damage: Some(25),
                self_damage: Some(5),
                target_heal: None,
                self_heal: None,
                self_damage_reduction: None,
            }),
        ],
        HeroClass::Cleric => vec![
            HeroAbility::Heal(AbilityEffects {
                target_damage: Some(25),
                self_damage: Some(5),
                target_heal: None,
                self_heal: None,
                self_damage_reduction: None,
            }),
            HeroAbility::Ultimate(AbilityEffects {
                target_damage: Some(25),
                self_damage: Some(5),
                target_heal: None,
                self_heal: None,
                self_damage_reduction: None,
            }),
        ],
    }
}
