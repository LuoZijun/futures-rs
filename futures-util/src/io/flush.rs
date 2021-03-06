use futures_core::future::Future;
use futures_core::task::{self, Poll};
use std::io;
use std::marker::Unpin;
use std::mem::PinMut;

use futures_io::AsyncWrite;

/// A future used to fully flush an I/O object.
///
/// Resolves to the underlying I/O object once the flush operation is complete.
///
/// Created by the [`flush`] function.
///
/// [`flush`]: fn.flush.html
#[derive(Debug)]
pub struct Flush<'a, W: ?Sized + 'a> {
    writer: &'a mut W,
}

// Pinning is never projected to fields
impl<W: ?Sized> Unpin for Flush<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized> Flush<'a, W> {
    pub(super) fn new(writer: &'a mut W) -> Self {
        Flush { writer }
    }
}

impl<W> Future for Flush<'_, W>
    where W: AsyncWrite + ?Sized,
{
    type Output = io::Result<()>;

    fn poll(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        self.writer.poll_flush(cx)
    }
}
