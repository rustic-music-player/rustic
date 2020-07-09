use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rating {
    None,
    Like,
    Dislike,
    Stars(u8),
}
