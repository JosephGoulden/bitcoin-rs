use crate::rpc_server::Dependencies;
use crate::v1::*;
use crate::MetaIoHandler;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Api {
	Control,
	Generate,
	Raw,
	Miner,
	BlockChain,
	Network,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApiSet {
	List(HashSet<Api>),
}

impl Default for ApiSet {
	fn default() -> Self {
		ApiSet::List(
			vec![Api::Control, Api::Generate, Api::Raw, Api::Miner, Api::BlockChain, Api::Network]
				.into_iter()
				.collect(),
		)
	}
}

impl FromStr for Api {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"control" => Ok(Api::Control),
			"generate" => Ok(Api::Generate),
			"raw" => Ok(Api::Raw),
			"miner" => Ok(Api::Miner),
			"blockchain" => Ok(Api::BlockChain),
			"network" => Ok(Api::Network),
			api => Err(format!("Unknown api: {}", api)),
		}
	}
}

impl ApiSet {
	pub fn list_apis(&self) -> HashSet<Api> {
		match *self {
			ApiSet::List(ref apis) => apis.clone(),
		}
	}
}

pub fn setup_rpc(mut handler: MetaIoHandler<()>, apis: ApiSet, deps: Dependencies) -> MetaIoHandler<()> {
	for api in apis.list_apis() {
		match api {
			Api::Control => handler
				.extend_with(ControlClient::new(ControlClientCore::new(deps.memory.clone(), deps.shutdown_signal.clone())).to_delegate()),
			Api::Generate => handler.extend_with(GenerateClient::new(GenerateClientCore::new(deps.local_sync_node.clone())).to_delegate()),
			Api::Raw => handler.extend_with(
				RawClient::new(RawClientCore::new(deps.network, deps.local_sync_node.clone(), deps.storage.clone())).to_delegate(),
			),
			Api::Miner => handler.extend_with(MinerClient::new(MinerClientCore::new(deps.local_sync_node.clone())).to_delegate()),
			Api::BlockChain => handler.extend_with(
				BlockChainClient::new(BlockChainClientCore::new(
					deps.network,
					deps.storage.clone(),
					Some(deps.local_sync_node.clone()),
				))
				.to_delegate(),
			),
			Api::Network => handler.extend_with(NetworkClient::new(NetworkClientCore::new(deps.p2p_context.clone())).to_delegate()),
		}
	}

	handler
}
