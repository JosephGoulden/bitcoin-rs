use message::{MessageHeader, MessageResult};
use network::Magic;
use crate::io::SharedTcpStream;

pub async fn read_header(a: &SharedTcpStream, magic: Magic) -> MessageResult<MessageHeader> {
	let mut buf= [0u8; 24];
	a.read_exact(&mut buf).await;
	MessageHeader::deserialize(&buf, magic)
}

#[cfg(test)]
mod tests {
	use super::read_header;
	use message::{Error, MessageHeader};
	use network::Network;
	use crate::bytes::Bytes;
	use crate::io::shared_tcp_stream::SharedTcpStream;

	#[tokio::test]
	async fn test_read_header() {
		let stream = SharedTcpStream::new("f9beb4d96164647200000000000000001f000000ed52399b".into());
		let expected = MessageHeader {
			magic: Network::Mainnet.magic(),
			command: "addr".into(),
			len: 0x1f,
			checksum: "ed52399b".into(),
		};

		assert_eq!(read_header(&stream, Network::Mainnet.magic()).await, Ok(expected));
		assert_eq!(
			read_header(&stream, Network::Testnet.magic()).await,
			Err(Error::InvalidMagic)
		);
	}

	#[tokio::test]
	async fn test_read_header_with_invalid_magic() {
		let stream = SharedTcpStream::new("f9beb4d86164647200000000000000001f000000ed52399b".into());
		assert_eq!(
			read_header(&stream, Network::Testnet.magic()).await,
			Err(Error::InvalidMagic)
		);
	}

	#[tokio::test]
	async fn test_read_too_short_header() {
		let stream = SharedTcpStream::new("f9beb4d96164647200000000000000001f000000ed5239".into());
		assert!(read_header(&stream, Network::Mainnet.magic()).await.is_err());
	}
}
