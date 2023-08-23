use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericResponse<T>{
    pub message: String,
    pub data: Option<T>,
}