use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub enum GetNormalInfoParams {
    Bvid(String),
    Aid(i64),
}

impl GetNormalInfoParams {
    pub fn get_bvid(&self) -> Option<String> {
        match self {
            Self::Bvid(bvid) => Some(bvid.clone()),
            Self::Aid(_) => None,
        }
    }

    pub fn get_aid(&self) -> Option<i64> {
        match self {
            Self::Bvid(_) => None,
            Self::Aid(aid) => Some(*aid),
        }
    }
}
