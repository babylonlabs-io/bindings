use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct BtcBlockHeader {
    pub version: i32,
    // btc compatible (serialized in reverse byte order) hex encoded hash of previous block
    pub prev_blockhash: String,
    // btc compatible (serialized in reverse byte order) hex encoded merkle root of transactions
    pub merkle_root: String,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

#[cw_serde]
pub struct BtcBlockHeaderInfo {
    pub header: BtcBlockHeader,
    pub height: u64,
}
