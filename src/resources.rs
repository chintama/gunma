use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub jump: bool,
    pub left: bool,
    pub right: bool,
    pub take: bool,
    pub drop: bool,
}

impl Action {
    pub fn clear(&mut self) -> Self {
        std::mem::replace(self, Self::default())
    }
}
