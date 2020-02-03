use crypto::checksum;
use message::{Error, MessageResult, Command};
use network::Magic;
use crate::io::{read_header, SharedTcpStream};
use crate::bytes::Bytes;

pub async fn read_any_message(a: &SharedTcpStream, magic: Magic) -> MessageResult<(Command, Bytes)> {
	let header = read_header(&a, magic).await?;

	let mut buf = Bytes::new_with_len(header.len as usize);
	a.read_exact(buf.as_mut()).await.expect("error reading from stream");

	if checksum(&buf) != header.checksum {
		return Err(Error::InvalidChecksum).into();
	}
	Ok((header.command.clone(), buf.into()))
}

#[cfg(test)]
mod tests {
	use super::read_any_message;
	use message::Error;
	use network::Network;
	use crate::bytes::Bytes;
	use crate::io::shared_tcp_stream::SharedTcpStream;

	#[tokio::test]
	async fn test_read_any_message() {
		let stream = SharedTcpStream::new("f9beb4d970696e6700000000000000000800000083c00c765845303b6da97786".into());
		let name = "ping".into();
		let nonce = "5845303b6da97786".into();
		let expected = (name, nonce);

		assert_eq!(
			read_any_message(&stream, Network::Mainnet.magic()).await,
			Ok(expected)
		);
		assert_eq!(
			read_any_message(&stream, Network::Testnet.magic()).await,
			Err(Error::InvalidMagic)
		);
	}

	#[tokio::test]
	async fn test_read_too_short_any_message() {
		let stream = SharedTcpStream::new("f9beb4d970696e6700000000000000000800000083c00c765845303b6da977".into());
		assert!(read_any_message(&stream, Network::Mainnet.magic()).await.is_err());
	}

	#[tokio::test]
	async fn test_read_any_message_with_invalid_checksum() {
		let stream = SharedTcpStream::new("f9beb4d970696e6700000000000000000800000083c01c765845303b6da97786".into());

		assert_eq!(
			read_any_message(&stream, Network::Mainnet.magic()).await,
			Err(Error::InvalidChecksum)
		);
	}
}
