//use futures::{Async, Future, Poll, Select, FutureExt};
use std::io;
use std::time::Duration;
use std::future::Future;
use tokio::time::{timeout};

pub async fn deadline<F, T>(duration: Duration, future: F) -> Result<DeadlineStatus<T>, io::Error>
where
	F: Future<Output = T> + Send,
{
	match timeout(duration, future).await {
		Ok(value) => Ok(DeadlineStatus::Meet(value)),
		_ => Ok(DeadlineStatus::Timeout)
	}
}

pub enum DeadlineStatus<T> {
	Meet(T),
	Timeout,
}
