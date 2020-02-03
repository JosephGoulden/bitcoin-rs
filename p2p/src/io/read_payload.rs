use message::{deserialize_payload, Error, MessageResult, Payload};
use crate::bytes::Bytes;
use crate::hash::H32;
use crate::io::SharedTcpStream;

pub async fn read_payload<M>(a: &SharedTcpStream, version: u32, len: usize, checksum: H32) -> MessageResult<M>
where
	M: Payload,
{
	let mut buf = Bytes::new_with_len(len);
	a.read_exact(buf.as_mut()).await;
	if crypto::checksum(&buf) != checksum {
		return Err(Error::InvalidChecksum);
	}
	deserialize_payload(&buf, version)
}

#[cfg(test)]
mod tests {
	use super::read_payload;
	use message::types::Ping;
	use message::Error;
	use crate::bytes::Bytes;
	use crate::io::shared_tcp_stream::SharedTcpStream;

	#[tokio::test]
	async fn test_read_payload() {
		let stream = SharedTcpStream::new("5845303b6da97786".into());
		let ping = Ping::new(u64::from_str_radix("8677a96d3b304558", 16).unwrap());
		assert_eq!(read_payload(&stream, 0, 8, "83c00c76".into()).await, Ok(ping));
	}

	#[tokio::test]
	async fn test_read_payload_with_invalid_checksum() {
		let stream = SharedTcpStream::new("5845303b6da97786".into());
		assert_eq!(
			read_payload::<Ping>(&stream, 0, 8, "83c00c75".into()).await,
			Err(Error::InvalidChecksum)
		);
	}

	#[tokio::test]
	async fn test_read_too_short_payload() {
		let stream = SharedTcpStream::new("5845303b6da977".into());
		assert!(read_payload::<Ping>(&stream, 0, 8, "83c00c76".into()).await.is_err());
	}
}
