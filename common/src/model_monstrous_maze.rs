use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonstrousMazeOutput {
    pub path: String
}
