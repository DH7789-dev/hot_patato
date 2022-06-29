use serde::{Serialize, Deserialize};
use crate::SubscribeError::SubscribeError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeResult {
    result: Result<T, SubscribeError>
}

enum Result<T, SubscribeError> {
    Ok(T),
    Err(SubscribeError),
}