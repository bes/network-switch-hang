use std::io::Result;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use pin_project::pin_project;
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::{interval, Interval};

// Code taken from https://stackoverflow.com/questions/60621835/how-to-get-callback-update-when-using-tokioiocopy?rq=1

#[pin_project]
pub struct ProgressReadAdapter<R: AsyncRead> {
    #[pin]
    inner: R,
    interval: Interval,
    interval_bytes: usize,
}

impl<R: AsyncRead> ProgressReadAdapter<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            interval: interval(Duration::from_millis(100)),
            interval_bytes: 0,
        }
    }
}

impl<R: AsyncRead> AsyncRead for ProgressReadAdapter<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        let this = self.project();

        let before = buf.filled().len();
        let result = this.inner.poll_read(cx, buf);
        let after = buf.filled().len();

        *this.interval_bytes += after - before;
        match this.interval.poll_tick(cx) {
            Poll::Pending => {}
            Poll::Ready(_) => {
                println!("reading at {} bytes per second", *this.interval_bytes * 10);
                *this.interval_bytes = 0;
            }
        };

        result
    }
}
