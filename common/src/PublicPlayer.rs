use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    name: str,
    stream_id: str,
    score: str,
    steps: str,
    is_active: bool,
    total_used_time: f64
}