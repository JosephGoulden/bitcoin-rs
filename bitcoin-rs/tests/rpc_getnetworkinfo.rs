use message::common::InventoryVector;
use message::types::{GetData, SendHeaders, Headers};
use network::Network;
use node_manager::NodeManager;
use test_data::block_h0;

const BITCOIN_RS: &str = env!("CARGO_BIN_EXE_bitcoin-rs");

#[tokio::test]
async fn test_rpc_getmemoryinfo() {
	let bitcoin_rs = NodeManager::new_node(BITCOIN_RS, "getmemoryinfo").start();

	bitcoin_rs.rpc.mem
}
