use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub enum GetCheeseInfoParams {
    EpId(i64),
    SeasonId(i64),
}

impl GetCheeseInfoParams {
    pub fn get_ep_id(&self) -> Option<i64> {
        match self {
            Self::EpId(ep_id) => Some(*ep_id),
            Self::SeasonId(_) => None,
        }
    }

    pub fn get_season_id(&self) -> Option<i64> {
        match self {
            Self::EpId(_) => None,
            Self::SeasonId(season_id) => Some(*season_id),
        }
    }
}
