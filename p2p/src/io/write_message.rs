use message::Message;
use crate::io::SharedTcpStream;

pub async fn write_message<M>(a: &SharedTcpStream, message: Message<M>)
{
	a.write_all(message.as_ref()).await;
}
