use message::{Error, MessageResult, Payload};
use network::Magic;
use crate::io::{read_header, read_payload, SharedTcpStream};

pub async fn read_message<M>(a: &SharedTcpStream, magic: Magic, version: u32) -> MessageResult<M>
where
	M: Payload,
{
	let header = read_header(a, magic).await?;
	if header.command != M::command() {
		return Err(Error::InvalidCommand).into();
	}
	read_payload(a, version, header.len as usize, header.checksum).await
}

#[cfg(test)]
mod tests {
	use super::read_message;
	use message::types::{Ping, Pong};
	use message::Error;
	use network::Network;
	use crate::bytes::Bytes;
	use crate::io::shared_tcp_stream::SharedTcpStream;

	#[tokio::test]
	async fn test_read_message() {
		let stream = SharedTcpStream::new("f9beb4d970696e6700000000000000000800000083c00c765845303b6da97786".into());
		let ping = Ping::new(u64::from_str_radix("8677a96d3b304558", 16).unwrap());
		assert_eq!(read_message(&stream, Network::Mainnet.magic(), 0).await, Ok(ping));
		assert_eq!(
			read_message::<Ping>(&stream, Network::Testnet.magic(), 0).await,
			Err(Error::InvalidMagic)
		);
		assert_eq!(
			read_message::<Pong>(&stream, Network::Mainnet.magic(), 0).await,
			Err(Error::InvalidCommand)
		);
	}

	#[tokio::test]
	async fn test_read_too_short_message() {
		let stream = SharedTcpStream::new("f9beb4d970696e6700000000000000000800000083c00c765845303b6da977".into());
		assert!(read_message::<Ping>(&stream, Network::Mainnet.magic(), 0).await.is_err());
	}

	#[tokio::test]
	async fn test_read_message_with_invalid_checksum() {
		let stream = SharedTcpStream::new("f9beb4d970696e6700000000000000000800000083c01c765845303b6da97786".into());
		assert_eq!(
			read_message::<Ping>(&stream, Network::Mainnet.magic(), 0).await,
			Err(Error::InvalidChecksum)
		);
	}
}
