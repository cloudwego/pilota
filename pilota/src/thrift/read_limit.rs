use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};

use super::{ProtocolException, ProtocolExceptionKind, ThriftException, new_protocol_exception};

pub(super) fn check_remaining(remaining: usize, length: usize) -> Result<(), ThriftException> {
    if length > remaining {
        return Err(new_protocol_exception(
            ProtocolExceptionKind::SizeLimit,
            "field exceeds remaining message byte limit",
        ));
    }
    Ok(())
}

/// Counts bytes delivered to the protocol, rather than bytes prefetched by its
/// reader.
pub(super) struct ReadLimit<R> {
    inner: R,
    remaining: Option<usize>,
}

impl<R> ReadLimit<R> {
    pub(super) fn new(inner: R, remaining: Option<usize>) -> Self {
        Self { inner, remaining }
    }

    pub(super) fn remaining(&self) -> Option<usize> {
        self.remaining
    }

    pub(super) fn check(&self, length: usize) -> Result<(), ThriftException> {
        if let Some(remaining) = self.remaining {
            check_remaining(remaining, length)?;
        }
        Ok(())
    }

    pub(super) fn check_container(
        &self,
        count: usize,
        per_element: usize,
    ) -> Result<(), ThriftException> {
        let minimum = count.checked_mul(per_element).ok_or_else(|| {
            new_protocol_exception(ProtocolExceptionKind::SizeLimit, "container size overflow")
        })?;
        self.check(minimum)
    }
}

impl<R: AsyncRead + Unpin> ReadLimit<R> {
    pub(super) async fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, ThriftException> {
        self.check(length)?;
        const INITIAL_CAPACITY: usize = 64 * 1024;
        let mut bytes = if length <= INITIAL_CAPACITY {
            vec![0; length]
        } else {
            Vec::with_capacity(INITIAL_CAPACITY)
        };
        if length <= INITIAL_CAPACITY {
            self.read_exact(&mut bytes).await?;
        } else {
            // Grow with received data, rather than a peer's declared length.
            self.take(length as u64).read_to_end(&mut bytes).await?;
            if bytes.len() != length {
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof).into());
            }
        }
        Ok(bytes)
    }

    pub(super) async fn discard(&mut self, mut length: usize) -> Result<(), ThriftException> {
        self.check(length)?;
        // Only the binary leaf owns this buffer; recursive skip futures stay small.
        Box::pin(async {
            let mut scratch = [0; 1024];
            while length != 0 {
                let chunk = length.min(scratch.len());
                self.read_exact(&mut scratch[..chunk]).await?;
                length -= chunk;
            }
            Ok(())
        })
        .await
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for ReadLimit<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        let Some(remaining) = this.remaining else {
            return Pin::new(&mut this.inner).poll_read(cx, buf);
        };
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }
        if remaining == 0 {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidData,
                ProtocolException::new(
                    ProtocolExceptionKind::SizeLimit,
                    "message byte limit exceeded",
                ),
            )));
        }
        let capacity = remaining.min(buf.remaining());
        let mut limited = buf.take(capacity);
        match Pin::new(&mut this.inner).poll_read(cx, &mut limited) {
            Poll::Ready(Ok(())) => {
                let read = limited.filled().len();
                // AsyncRead guarantees that the filled region is initialized.
                unsafe { buf.assume_init(read) };
                buf.advance(read);
                this.remaining = Some(remaining - read);
                Poll::Ready(Ok(()))
            }
            result => result,
        }
    }
}
