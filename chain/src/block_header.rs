use crate::compact::Compact;
use bitcrypto::{FromHex, Hash, SHA256D};
use ser::{deserialize, serialize};
use std::fmt;

#[derive(PartialEq, Clone, Serializable, Deserializable)]
pub struct BlockHeader {
	pub version: u32,
	pub previous_header_hash: SHA256D,
	pub merkle_root_hash: SHA256D,
	pub time: u32,
	pub bits: Compact,
	pub nonce: u32,
}

impl BlockHeader {
	/// Compute hash of the block header.
	#[cfg(any(test, feature = "test-helpers"))]
	pub fn hash(&self) -> SHA256D {
		block_header_hash(self)
	}
}

impl fmt::Debug for BlockHeader {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BlockHeader")
			.field("version", &self.version)
			.field("previous_header_hash", &self.previous_header_hash)
			.field("merkle_root_hash", &self.merkle_root_hash)
			.field("time", &self.time)
			.field("bits", &self.bits)
			.field("nonce", &self.nonce)
			.finish()
	}
}

impl From<&'static str> for BlockHeader {
	fn from(s: &'static str) -> Self {
		let hex: Vec<u8> = FromHex::from_hex(s).unwrap();
		deserialize(&*hex).unwrap()
	}
}

/// Compute hash of the block header.
pub(crate) fn block_header_hash(block_header: &BlockHeader) -> SHA256D {
	SHA256D::hash(&serialize(block_header))
}

#[cfg(test)]
mod tests {
	use super::BlockHeader;
	use bitcrypto::{Hash, SHA256D};
	use ser::{Error as ReaderError, Reader, Stream};

	#[test]
	fn test_block_header_stream() {
		let block_header = BlockHeader {
			version: 1,
			previous_header_hash: SHA256D::from_inner([2; 32]),
			merkle_root_hash: SHA256D::from_inner([3; 32]),
			time: 4,
			bits: 5.into(),
			nonce: 6,
		};

		let mut stream = Stream::default();
		stream.append(&block_header);
		#[rustfmt::skip]
		let expected = vec![
			1, 0, 0, 0,
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			4, 0, 0, 0,
			5, 0, 0, 0,
			6, 0, 0, 0,
		].into();

		assert_eq!(stream.out(), expected);
	}

	#[test]
	fn test_block_header_reader() {
		#[rustfmt::skip]
		let buffer = vec![
			1, 0, 0, 0,
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			4, 0, 0, 0,
			5, 0, 0, 0,
			6, 0, 0, 0,
		];

		let mut reader = Reader::new(&buffer);

		let expected = BlockHeader {
			version: 1,
			previous_header_hash: SHA256D::from_inner([2; 32]),
			merkle_root_hash: SHA256D::from_inner([3; 32]),
			time: 4,
			bits: 5.into(),
			nonce: 6,
		};

		assert_eq!(expected, reader.read().unwrap());
		assert_eq!(ReaderError::UnexpectedEnd, reader.read::<BlockHeader>().unwrap_err());
	}
}
