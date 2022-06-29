use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HD5HashCashInput {
    complexity: u32,
    message: str
}