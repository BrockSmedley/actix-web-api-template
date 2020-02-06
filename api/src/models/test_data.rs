use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestData {
    pub val: u64,
    pub name: String,
}
