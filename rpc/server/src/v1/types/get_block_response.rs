use super::block::RawBlock;
use super::uint::U256;
use bitcrypto::SHA256D;
use serde::{Serialize, Serializer};

/// Response to getblock RPC request
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetBlockResponse {
	/// When asking for short response
	Raw(RawBlock),
	/// When asking for verbose response
	Verbose(VerboseBlock),
}

/// Verbose block information
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct VerboseBlock {
	/// Block hash
	pub hash: SHA256D,
	/// Number of confirmations. -1 if block is on the side chain
	pub confirmations: i64,
	/// Block size
	pub size: u32,
	/// Block size, excluding witness data
	pub strippedsize: u32,
	/// Block weight
	pub weight: u32,
	/// Block height
	/// TODO: bitcoind always returns value, but we hold this value for main chain blocks only
	pub height: Option<u32>,
	/// Block version
	pub version: u32,
	/// Block version as hex
	#[serde(rename = "versionHex")]
	pub version_hex: String,
	/// Merkle root of this block
	pub merkleroot: SHA256D,
	/// Transaction ids
	pub tx: Vec<SHA256D>,
	/// Block time in seconds since epoch (Jan 1 1970 GMT)
	pub time: u32,
	/// Median block time in seconds since epoch (Jan 1 1970 GMT)
	/// TODO: bitcoind always returns value, but we can calculate this only if height(block) > 2
	pub mediantime: Option<u32>,
	/// Block nonce
	pub nonce: u32,
	/// Block nbits
	pub bits: u32,
	/// Block difficulty
	pub difficulty: f64,
	/// Expected number of hashes required to produce the chain up to this block (in hex)
	pub chainwork: U256,
	/// Hash of previous block
	pub previousblockhash: Option<SHA256D>,
	/// Hash of next block
	pub nextblockhash: Option<SHA256D>,
}

impl Serialize for GetBlockResponse {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match *self {
			GetBlockResponse::Raw(ref raw_block) => raw_block.serialize(serializer),
			GetBlockResponse::Verbose(ref verbose_block) => verbose_block.serialize(serializer),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::bytes::Bytes;
	use super::super::uint::U256;
	use super::*;
	use bitcrypto::{FromInnerHex, SHA256D};
	use serde_json;

	#[test]
	fn verbose_block_serialize() {
		let block = VerboseBlock::default();
		assert_eq!(
			serde_json::to_string(&block).unwrap(),
			r#"{"hash":"0000000000000000000000000000000000000000000000000000000000000000","confirmations":0,"size":0,"strippedsize":0,"weight":0,"height":null,"version":0,"versionHex":"","merkleroot":"0000000000000000000000000000000000000000000000000000000000000000","tx":[],"time":0,"mediantime":null,"nonce":0,"bits":0,"difficulty":0.0,"chainwork":"0000000000000000000000000000000000000000000000000000000000000000","previousblockhash":null,"nextblockhash":null}"#
		);

		let block = VerboseBlock {
			hash: SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
			confirmations: -1,
			size: 500000,
			strippedsize: 444444,
			weight: 5236235,
			height: Some(3513513),
			version: 1,
			version_hex: "01".to_owned(),
			merkleroot: SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
			tx: vec![
				SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000003").unwrap(),
				SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000004").unwrap(),
			],
			time: 111,
			mediantime: Some(100),
			nonce: 124,
			bits: 13513,
			difficulty: 555.555,
			chainwork: U256::default(),
			previousblockhash: Some(SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000004").unwrap()),
			nextblockhash: Some(SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000005").unwrap()),
		};
		assert_eq!(
			serde_json::to_string(&block).unwrap(),
			r#"{"hash":"0100000000000000000000000000000000000000000000000000000000000000","confirmations":-1,"size":500000,"strippedsize":444444,"weight":5236235,"height":3513513,"version":1,"versionHex":"01","merkleroot":"0200000000000000000000000000000000000000000000000000000000000000","tx":["0300000000000000000000000000000000000000000000000000000000000000","0400000000000000000000000000000000000000000000000000000000000000"],"time":111,"mediantime":100,"nonce":124,"bits":13513,"difficulty":555.555,"chainwork":"0000000000000000000000000000000000000000000000000000000000000000","previousblockhash":"0400000000000000000000000000000000000000000000000000000000000000","nextblockhash":"0500000000000000000000000000000000000000000000000000000000000000"}"#
		);
	}

	#[test]
	fn verbose_block_deserialize() {
		let block = VerboseBlock::default();
		assert_eq!(
			serde_json::from_str::<VerboseBlock>(r#"{"hash":"0000000000000000000000000000000000000000000000000000000000000000","confirmations":0,"size":0,"strippedsize":0,"weight":0,"height":null,"version":0,"versionHex":"","merkleroot":"0000000000000000000000000000000000000000000000000000000000000000","tx":[],"time":0,"mediantime":null,"nonce":0,"bits":0,"difficulty":0.0,"chainwork":"0000000000000000000000000000000000000000000000000000000000000000","previousblockhash":null,"nextblockhash":null}"#).unwrap(),
			block);

		let block = VerboseBlock {
			hash: SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
			confirmations: -1,
			size: 500000,
			strippedsize: 444444,
			weight: 5236235,
			height: Some(3513513),
			version: 1,
			version_hex: "01".to_owned(),
			merkleroot: SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
			tx: vec![
				SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000003").unwrap(),
				SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000004").unwrap(),
			],
			time: 111,
			mediantime: Some(100),
			nonce: 124,
			bits: 13513,
			difficulty: 555.555,
			chainwork: U256::default(),
			previousblockhash: Some(SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000004").unwrap()),
			nextblockhash: Some(SHA256D::from_inner_hex("0000000000000000000000000000000000000000000000000000000000000005").unwrap()),
		};
		assert_eq!(
			serde_json::from_str::<VerboseBlock>(r#"{"hash":"0100000000000000000000000000000000000000000000000000000000000000","confirmations":-1,"size":500000,"strippedsize":444444,"weight":5236235,"height":3513513,"version":1,"versionHex":"01","merkleroot":"0200000000000000000000000000000000000000000000000000000000000000","tx":["0300000000000000000000000000000000000000000000000000000000000000","0400000000000000000000000000000000000000000000000000000000000000"],"time":111,"mediantime":100,"nonce":124,"bits":13513,"difficulty":555.555,"chainwork":"0000000000000000000000000000000000000000000000000000000000000000","previousblockhash":"0400000000000000000000000000000000000000000000000000000000000000","nextblockhash":"0500000000000000000000000000000000000000000000000000000000000000"}"#).unwrap(),
			block);
	}

	#[test]
	fn get_block_response_raw_serialize() {
		let raw_response = GetBlockResponse::Raw(Bytes::new(vec![0]));
		assert_eq!(serde_json::to_string(&raw_response).unwrap(), r#""00""#);
	}

	#[test]
	fn get_block_response_verbose_serialize() {
		let block = VerboseBlock::default();
		let verbose_response = GetBlockResponse::Verbose(block);
		assert_eq!(
			serde_json::to_string(&verbose_response).unwrap(),
			r#"{"hash":"0000000000000000000000000000000000000000000000000000000000000000","confirmations":0,"size":0,"strippedsize":0,"weight":0,"height":null,"version":0,"versionHex":"","merkleroot":"0000000000000000000000000000000000000000000000000000000000000000","tx":[],"time":0,"mediantime":null,"nonce":0,"bits":0,"difficulty":0.0,"chainwork":"0000000000000000000000000000000000000000000000000000000000000000","previousblockhash":null,"nextblockhash":null}"#
		);
	}
}
