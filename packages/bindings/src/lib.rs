mod querier;
mod query;

pub use querier::BabylonQuerier;
pub use query::{BabylonQuery, CurrentEpochResponse};
// This export is added to all contracts that import this package, signifying that they require
// "babylon" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_babylon() {}
