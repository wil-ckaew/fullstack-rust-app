use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: Option<i32>,
    pub user_id: i32,
    pub filename: String,
    pub data: Vec<u8>, // Representa o BYTEA como um vetor de bytes
}
