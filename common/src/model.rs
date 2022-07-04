use serde::{Deserialize, Serialize};
use crate::model_md5_challenge::{MD5HashCashInput, MD5HashCashOutput};
use crate::model_monstrous_maze::{MonstrousMazeInput, MonstrousMazeOutput};
use crate::model_recover_secret::{RecoverSecretInput, RecoverSecretOutput};

#[derive(Serialize, Debug)]
pub struct Hello {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub version: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    InvalidName,
    Timeout,
    AlreadyRegistered
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicLeaderBoard {
    pub public_leaderboard: Vec<PublicPlayer>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
    MonstrousMaze(MonstrousMazeInput),
    RecoverSecret(RecoverSecretInput),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    MonstrousMaze(MonstrousMazeOutput),
    RecoverSecret(RecoverSecretOutput)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Ok { used_time: f64, next_target: String },
    BadResult { used_time: f64, next_target: String},
    Timeout
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundSummary {
    pub challenge: String,
    pub chain: Vec<ReportedChallengeResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndOfGame {
    leader_board: Vec<PublicPlayer>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JsonMessage {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}

