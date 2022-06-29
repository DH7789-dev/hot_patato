use serde::{Serialize, Deserialize};
use crate::PublicPlayer::PublicPlayer;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard {
    players: Vec<PublicPlayer>
}