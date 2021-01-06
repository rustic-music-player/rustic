use std::pin::Pin;
use std::task::{Context, Poll};

use crossbeam_channel::{Receiver, Sender, TryRecvError};
use futures::Stream;
use std::thread;
use std::fmt::Debug;

pub fn from_channel<T>(recv: Receiver<T>) -> StreamingReceiver<T> {
    let (inner_tx, inner_rx) = crossbeam_channel::unbounded();

    StreamingReceiver {
        inner_rx,
        inner_tx,
        outer_rx: recv,
    }
}

pub struct StreamingReceiver<T> {
    inner_rx: Receiver<T>,
    inner_tx: Sender<T>,
    outer_rx: Receiver<T>,
}

impl<T: Send + 'static + Debug> Stream for StreamingReceiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let receiver = self.outer_rx.clone();
        let sender = self.inner_tx.clone();
        let waker = cx.waker().clone();
        // TODO: use thread pool or tokio runtime? replace crossbeam channels with async native channels?
        thread::spawn(move || {
            let result = receiver.recv();
            log::trace!("streaming receiver waker: {:?}", result);
            if let Ok(value) = result {
                sender.send(value).unwrap();
                waker.wake();
            }
        });
        let result = self.inner_rx.try_recv();
        log::trace!("streaming receiver: {:?}", result);
        match result {
            Ok(item) => Poll::Ready(Some(item)),
            Err(TryRecvError::Empty) => Poll::Pending,
            Err(TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}
