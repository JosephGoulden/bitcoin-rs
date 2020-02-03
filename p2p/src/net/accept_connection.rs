use std::time::Duration;
use std::{io, net};
use tokio::net::TcpStream;
use crate::io::{deadline, accept_handshake, DeadlineStatus, SharedTcpStream};
use crate::net::{Connection, Config};

pub async fn accept_connection<'a>(stream: TcpStream, config: &Config, address: net::SocketAddr) -> Result<DeadlineStatus<Connection>, io::Error> {
	let shared_stream: SharedTcpStream = stream.into();
	let accept = async {
		let handshake_result =
			accept_handshake(&shared_stream, config.magic, config.version(&address), config.protocol_minimum).await.unwrap();

		Connection {
			stream: shared_stream,
			services: handshake_result.version.services(),
			version: handshake_result.negotiated_version,
			version_message: handshake_result.version,
			magic: config.magic,
			address,
		}
	};

	deadline(Duration::new(5, 0), accept).await
}
