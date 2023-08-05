use std::ops::Not;
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



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardColor {
    RED,
    GREEN,
}
impl Not for CardColor {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            CardColor::RED => CardColor::GREEN,
            CardColor::GREEN => CardColor::RED,
        }
    }
}

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, PartialOrd)]
pub enum Size {
    BIG = 2,
    MID = 1,
    SMALL = 0,
}
