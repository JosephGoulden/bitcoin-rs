extern crate bitcrypto as crypto;
extern crate byteorder;
extern crate chain;
#[macro_use]
extern crate primitives;
#[macro_use]
extern crate serialization as ser;
#[macro_use]
extern crate serialization_derive;
extern crate network;

pub mod common;
mod error;
mod message;
mod serialization;
pub mod types;

pub use primitives::{bytes, hash};

pub use crate::message::{to_raw_message, Message, MessageHeader, Payload};
pub use crate::serialization::{deserialize_payload, serialize_payload};
pub use common::{Command, Services};
pub use error::{Error, MessageResult};
