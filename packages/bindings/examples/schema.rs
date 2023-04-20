use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use babylon_bindings::{
    BabylonQuery, BtcBaseHeaderResponse, BtcBlockHeader, BtcBlockHeaderInfo,
    BtcHeaderByQueryResponse, BtcTipResponse, CurrentEpochResponse, LatestFinalizedEpochResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(BabylonQuery), &out_dir);
    export_schema(&schema_for!(CurrentEpochResponse), &out_dir);
    export_schema(&schema_for!(LatestFinalizedEpochResponse), &out_dir);
    export_schema(&schema_for!(BtcTipResponse), &out_dir);
    export_schema(&schema_for!(BtcBaseHeaderResponse), &out_dir);
    export_schema(&schema_for!(BtcHeaderByQueryResponse), &out_dir);
    export_schema(&schema_for!(BtcBlockHeaderInfo), &out_dir);
    export_schema(&schema_for!(BtcBlockHeader), &out_dir);
}
