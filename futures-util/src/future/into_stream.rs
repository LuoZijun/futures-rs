use core::mem::PinMut;
use futures_core::future::Future;
use futures_core::stream::Stream;
use futures_core::task::{self, Poll};
use pin_utils::unsafe_pinned;

/// A type which converts a `Future` into a `Stream`
/// containing a single element.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct IntoStream<Fut: Future> {
    future: Option<Fut>
}

impl<Fut: Future> IntoStream<Fut> {
    unsafe_pinned!(future: Option<Fut>);

    pub(super) fn new(future: Fut) -> IntoStream<Fut> {
        IntoStream {
            future: Some(future)
        }
    }
}

impl<Fut: Future> Stream for IntoStream<Fut> {
    type Item = Fut::Output;

    fn poll_next(mut self: PinMut<Self>, cx: &mut task::Context) -> Poll<Option<Self::Item>> {
        let v = match self.future().as_pin_mut() {
            Some(fut) => {
                match fut.poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(v) => v
                }
            }
            None => return Poll::Ready(None),
        };

        PinMut::set(self.future(), None);
        Poll::Ready(Some(v))
    }
}
