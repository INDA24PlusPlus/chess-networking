use std::time::Duration;

use serde_derive::{Deserialize, Serialize};

// Serializeation and Deserializeation is done via MessagePack

#[derive(Debug, Serialize, Deserialize)]
pub struct Start {
    is_white: bool,
    name: String,
    fen: Option<String>,
    time: Option<Duration>,
    inc: Option<Duration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    // (0, 0) = a1, (7, 0) = h1
    from: (u8, u8),
    to: (u8, u8),
    promotion: Option<PromotionPiece>,
    forfeit: bool,
    ofer_draw: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ack {
    ok: bool,
    game_state: GameState,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PromotionPiece {
    Queen,
    Bishop,
    Knight,
    Rook
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameState {
    CheckMate,
    Draw,
}

impl TryFrom<&[u8]> for Start {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Start> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Start) -> Result<Self, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Move {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Move> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Move) -> Result<Self, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&value)
    }
}

impl TryFrom<&[u8]> for Ack {
    type Error = rmp_serde::decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(value)
    }
}

impl TryFrom<Ack> for Vec<u8> {
    type Error = rmp_serde::encode::Error;

    fn try_from(value: Ack) -> Result<Self, rmp_serde::encode::Error> {
        rmp_serde::to_vec(&value)
    }
}
