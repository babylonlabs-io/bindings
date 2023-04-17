use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum BabylonQuery {
    #[returns(CurrentEpochResponse)]
    Epoch {},

    #[returns(LatestFinalizedEpochResponse)]
    LatestFinalizedEpoch {},
}

#[cw_serde]
pub struct CurrentEpochResponse {
    pub epoch: u64,
}
#[cw_serde]
pub struct LatestFinalizedEpochResponse {
    pub epoch: u64,
}

impl CustomQuery for BabylonQuery {}
