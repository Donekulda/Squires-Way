use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SourceType {
    #[default]
    None,
    Local,
    Paradox,
    Steam,
}
