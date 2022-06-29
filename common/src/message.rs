use crate::welcome::Welcome;
use serde::{Serialize, Deserialize};
use crate::Subscribe::Subscribe;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe)
}