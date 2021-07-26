use crate::*;
/// Assume this file is `log_events.rs`
/// There is a `lib.rs` file where the primary struct
/// is called `Contract`
use near_sdk::{
    near_bindgen, serde::Serialize, serde_json, Balance, BlockHeight, EpochHeight, Gas,
};

const VERSION: &'static str = "0.0.1";

#[near_bindgen]
impl Contract {
    /// Returns the version of the log events
    pub fn log_events_version(&self) -> String {
        VERSION.to_string()
    }
}

#[allow(dead_code)]
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum LogLevel {
    Info,
    Warn,
    UserError,
    ContractError,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LogNeeds {
    pub gas: Option<Gas>,
    pub deposit: Option<Balance>,
    pub height: Option<BlockHeight>,
    pub epoch: Option<EpochHeight>,
    pub block_timestamp: Option<u64>,
    /// Catch-all, used for custom conditions yet unknown
    pub other: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LogEvent {
    pub level: LogLevel,
    /// Unique, brief, all-cap + underscores code like:
    /// "NO_PLAYER", "ABILITY_DEPRECATION_SOON"
    pub code: &'static str,
    /// These are contract-level, human-readable (English) instructions like:
    /// "you must attach 0.0023 NEAR as deposit for storage"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<String>,
    /// Non-human-readable list of needs before user/dApp should retry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<LogNeeds>,
}

#[allow(dead_code)]
impl LogEvent {
    pub fn new(level: LogLevel, code: &'static str) -> Self {
        Self {
            level,
            code,
            advice: None,
            needs: None,
        }
    }

    /// Helper function to add to `advice` field.
    /// Returns `LogEvent` so it can be chained.
    pub fn advice(&mut self, i: String) -> &mut Self {
        self.advice = Some(i);
        self
    }

    /// Helper function to add to `needs` field.
    /// Returns `LogEvent` so it can be chained.
    pub fn needs(&mut self, n: LogNeeds) -> &mut Self {
        self.needs = Some(n);
        self
    }

    /// Turns the LogLevel into a vector of bytes.
    /// Useful with contract panics, like:
    ///
    /// # Example
    ///
    /// ```
    /// env::panic(&LogEvent::new(
    ///   LogLevel::ContractError,
    ///   "OWNER_NOT_FOUND"
    /// ).b())
    /// ```
    pub fn b(&self) -> Vec<u8> {
        Vec::from(self.to_string())
    }
}

/// Useful in common `expect` usages.
///
/// # Example
///
/// ```
/// let val = self.my_map.get(&input).expect(
///   &LogEvent::new(
///     LogLevel::UserError,
///     "WRONG_TURN")
///   .advice(
///     String::from("Please wait until it's your turn,
///     check with 'is_my_turn' method.")
///   ).to_string()
/// );
/// ```
impl ToString for LogEvent {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .expect(r#"{"level":"ContractError","code":"LOGEVENT_TO_STRING"}"#)
    }
}
