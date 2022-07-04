use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretInput {
    pub word_count: usize,
    pub letters: String,
    pub tuple_sizes: Vec<usize>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String
}

