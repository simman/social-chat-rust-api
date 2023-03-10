use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResult<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IMHttpResult<T> {
    pub code: usize,
    pub message: String,
    pub data: Option<T>,
}
