use crate::bytes::Bytes;
use crate::{Error, MessageResult, Payload};
use ser::Stream;

pub fn serialize_payload<T>(t: &T, version: u32) -> MessageResult<Bytes>
where
	T: Payload,
{
	serialize_payload_with_flags(t, version, 0)
}

pub fn serialize_payload_with_flags<T>(t: &T, version: u32, serialization_flags: u32) -> MessageResult<Bytes>
where
	T: Payload,
{
	let mut stream = PayloadStream::new(version, serialization_flags);
	stream.append(t)?;
	Ok(stream.out())
}

pub struct PayloadStream {
	stream: Stream,
	version: u32,
}

impl PayloadStream {
	pub fn new(version: u32, serialization_flags: u32) -> Self {
		PayloadStream {
			stream: Stream::with_flags(serialization_flags),
			version,
		}
	}

	pub fn append<T>(&mut self, t: &T) -> MessageResult<()>
	where
		T: Payload,
	{
		if T::version() > self.version {
			return Err(Error::InvalidVersion);
		}

		t.serialize_payload(&mut self.stream, self.version)
	}

	pub fn out(self) -> Bytes {
		self.stream.out()
	}
}
