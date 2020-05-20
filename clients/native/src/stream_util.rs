use std::pin::Pin;
use std::task::{Context, Poll};

use crossbeam_channel::{Receiver, TryRecvError};
use futures::Stream;

pub fn from_channel<T>(recv: Receiver<T>) -> StreamingReceiver<T> {
    StreamingReceiver(recv)
}

pub struct StreamingReceiver<T>(Receiver<T>);

impl<T> Stream for StreamingReceiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let result = self.0.try_recv();
        let poll = match result {
            Ok(item) => Poll::Ready(Some(item)),
            Err(TryRecvError::Empty) => {
                // TODO: we should keep track of the wakers and only wake when a new event is emitted
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(TryRecvError::Disconnected) => Poll::Ready(None),
        };
        poll
    }
}
