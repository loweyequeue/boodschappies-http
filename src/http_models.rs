use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ArticleCreate {
    pub label: String,
    pub amount: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArticleUpdate {
    pub amount: usize,
    pub completed: bool,
}
