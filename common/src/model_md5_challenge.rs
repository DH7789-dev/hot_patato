use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    pub complexity: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}


