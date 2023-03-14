use std::io::{ErrorKind, Result};
use std::ops::Add;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::error::StalledError;
use pin_project::pin_project;
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::{interval_at, Instant, Interval};

/// This monitor can wrap an [AsyncRead] and make sure that it is making progress.
/// If the inner reader isn't making progress, we can stop the download.
/// The monitoring is done by keeping an [Interval] and measuring progress
/// by counting the number of bytes during each interval.
///
/// Please note that this monitor won't stop the download after _exactly_
/// five seconds of inactivity, but rather five seconds after the last interval
/// that had data. So the worst case is 10 seconds, and the averge will be 7.5 seconds.
#[pin_project]
pub struct StalledReadMonitor<R: AsyncRead> {
    #[pin]
    inner: R,
    interval: Interval,
    interval_bytes: usize,
}

impl<R: AsyncRead> StalledReadMonitor<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            interval: interval_at(
                Instant::now().add(Duration::from_millis(5_000)),
                Duration::from_millis(5_000),
            ),
            interval_bytes: 0,
        }
    }
}

impl<R: AsyncRead> AsyncRead for StalledReadMonitor<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let this = self.project();

        let before = buf.filled().len();
        let mut result = this.inner.poll_read(cx, buf);
        let after = buf.filled().len();

        *this.interval_bytes += after - before;
        match this.interval.poll_tick(cx) {
            Poll::Pending => {}
            Poll::Ready(_) => {
                if *this.interval_bytes == 0 {
                    println!("Rate is too low, aborting fetch");
                    result = Poll::Ready(Err(std::io::Error::new(
                        ErrorKind::TimedOut,
                        StalledError {},
                    )))
                }
                *this.interval_bytes = 0;
            }
        };
        result
    }
}
