use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SayHelloDTO {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct RepeatDTO {
    pub repeat_value: String,
}
