use crate::types::BtcBlockHeaderInfo;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

#[cw_serde]
#[derive(QueryResponses)]
pub enum BabylonQuery {
    #[returns(CurrentEpochResponse)]
    Epoch {},

    #[returns(LatestFinalizedEpochResponse)]
    LatestFinalizedEpoch {},

    #[returns(BtcBaseHeaderResponse)]
    BtcBaseHeader {},

    #[returns(BtcTipResponse)]
    BtcTip {},

    #[returns(BtcHeaderByQueryResponse)]
    BtcHeaderByNumber { height: u64 },

    #[returns(BtcHeaderByQueryResponse)]
    BtcHeaderByHash { hash: String },
}

#[cw_serde]
pub struct CurrentEpochResponse {
    pub epoch: u64,
}
#[cw_serde]
pub struct LatestFinalizedEpochResponse {
    pub epoch: u64,
}

#[cw_serde]
pub struct BtcTipResponse {
    pub header_info: BtcBlockHeaderInfo,
}

#[cw_serde]
pub struct BtcBaseHeaderResponse {
    pub header_info: BtcBlockHeaderInfo,
}
#[cw_serde]
pub struct BtcHeaderByQueryResponse {
    pub header_info: Option<BtcBlockHeaderInfo>,
}

impl CustomQuery for BabylonQuery {}
