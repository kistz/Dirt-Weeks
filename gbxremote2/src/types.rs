use serde::{Deserialize, Serialize};

use crate::ServerClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct WayPointEvent {
    #[serde(rename = "accountid")]
    account_id: String,
    login: String,
    time: u32,
    racetime: u32,
    laptime: u32,
    speed: f32,

    #[serde(rename = "checkpointinrace")]
    checkpoint_in_race: u32,
    #[serde(rename = "checkpointinlap")]
    checkpoint_in_lap: u32,
    #[serde(rename = "isendrace")]
    is_end_race: bool,
    #[serde(rename = "isendlap")]
    is_end_lap: bool,
    #[serde(rename = "isinfinitelaps")]
    is_infinite_laps: bool,
    #[serde(rename = "isindependentlaps")]
    is_independent_laps: bool,
    #[serde(rename = "curracecheckpoints")]
    current_race_checkpoints: Vec<u32>,
    #[serde(rename = "curlapcheckpoints")]
    current_lap_checkpoints: Vec<u32>,
    #[serde(rename = "blockid")]
    block_id: String,
}
