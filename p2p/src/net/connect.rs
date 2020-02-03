use std::io;
use std::net::{SocketAddr};
use std::time::Duration;
use crate::net::{Config, Connection};
use crate::io::{deadline, handshake, DeadlineStatus, SharedTcpStream};
use message::MessageResult;

pub async fn connect<'a>(address: & SocketAddr, config: &Config) -> Result<DeadlineStatus<MessageResult<Connection>>, io::Error> {

	let stream = SharedTcpStream::connect(address).await?;
	let connect = async {


		let handshake = handshake(&stream, config.magic, config.version(address), config.protocol_minimum).await?;

		Ok(Connection {
			stream,
			services: handshake.version.services(),
			version: handshake.negotiated_version,
			version_message: handshake.version,
			magic: config.magic,
			address: *address,
		})
	};

	deadline(Duration::new(5, 0), connect).await
}
