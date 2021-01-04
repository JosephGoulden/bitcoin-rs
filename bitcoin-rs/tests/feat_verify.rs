extern crate node_manager;
use node_manager::node;
use node_manager::NodeManager;
use tokio::time::Duration;

#[tokio::test]
async fn test_feat_verify() {
	let mut bitcoin_rs = node!();
	bitcoin_rs.with_sub_command("verify").start();

	let exit_status = bitcoin_rs.wait_for_exit(Duration::from_secs(3)).await.unwrap();
	assert!(exit_status.success());
}
