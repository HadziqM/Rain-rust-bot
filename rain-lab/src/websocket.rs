use serde::{Serialize, Deserialize};


#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum SocketComm {
    Command(String),
    Answer(String)
}
