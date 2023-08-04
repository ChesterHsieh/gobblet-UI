use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

pub const KEY_BEST_SCORE: &str = "memory.game.best.score";

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardName {
    RED_LARGE,
    RED_MEDIUM,
    RED_TINY,
    BLUE_TINY,
    BLUE_MEDIUM,
    BLUE_LARGE
}

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Ready,
    Playing,
    Passed,
}

pub const RAW_CARDS: [CardName; 12] = [
    CardName::RED_LARGE,
    CardName::RED_LARGE,
    CardName::RED_MEDIUM,
    CardName::RED_MEDIUM,
    CardName::RED_TINY,
    CardName::RED_TINY,
    CardName::BLUE_TINY,
    CardName::BLUE_TINY,
    CardName::BLUE_MEDIUM,
    CardName::BLUE_MEDIUM,
    CardName::BLUE_LARGE,
    CardName::BLUE_LARGE,
];


