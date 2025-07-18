use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Icon {
    pub path: String,
    pub size: (u32, u32),
}
