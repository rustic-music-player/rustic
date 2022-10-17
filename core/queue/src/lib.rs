use futures::stream::Stream;
use futures::stream::StreamExt;
use futures_signals::signal::{Mutable, MutableSignalCloned, SignalExt};

type SenderImplementation<T> = flume::Sender<T>;
type ReceiverImplementation<T> = flume::Receiver<T>;

pub type SendError<T> = flume::SendError<T>;
pub type RecvError = flume::RecvError;

pub fn broadcast<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = flume::unbounded();

    (tx.into(), rx.into())
}

pub fn one_shot<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = flume::bounded(1);

    (tx.into(), rx.into())
}

pub fn bus<T>() -> (BusSender<T>, BusReceiver<T>) {
    let mutable = Mutable::new(None);

    (mutable.clone().into(), mutable.into())
}

#[derive(Debug, Clone)]
pub struct Receiver<T> {
    rx: ReceiverImplementation<T>,
}

impl<T: 'static> Receiver<T> {
    pub fn stream(&self) -> impl Stream<Item = T> {
        self.rx.clone().into_stream()
    }

    pub async fn recv_async(&self) -> Result<T, RecvError> {
        self.rx.recv_async().await
    }
}

#[derive(Debug, Clone)]
pub struct Sender<T> {
    tx: SenderImplementation<T>,
}

impl<T> Sender<T> {
    pub fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.tx.send(msg)
    }

    pub async fn send_async(&self, msg: T) -> Result<(), SendError<T>> {
        self.tx.send_async(msg).await
    }
}

impl<T> From<SenderImplementation<T>> for Sender<T> {
    fn from(sender: SenderImplementation<T>) -> Sender<T> {
        Sender { tx: sender }
    }
}

impl<T> From<ReceiverImplementation<T>> for Receiver<T> {
    fn from(receiver: ReceiverImplementation<T>) -> Receiver<T> {
        Receiver { rx: receiver }
    }
}

#[derive(Debug, Clone)]
pub struct BusSender<T> {
    mutable: Mutable<Option<T>>
}

impl<T> BusSender<T> {
    pub fn send(&self, msg: T) -> Result<(), SendError<T>> {
        self.mutable.set(Some(msg));

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BusReceiver<T> {
    mutable: Mutable<Option<T>>
}

impl<T: 'static + Clone> BusReceiver<T> {
    pub fn stream(&self) -> impl Stream<Item = T> {
        self.mutable
            .signal_cloned()
            .to_stream()
            .filter_map(|item| async { item })
    }
}

impl<T> From<Mutable<Option<T>>> for BusSender<T> {
    fn from(mutable: Mutable<Option<T>>) -> Self {
        Self { mutable }
    }
}

impl<T> From<Mutable<Option<T>>> for BusReceiver<T> {
    fn from(mutable: Mutable<Option<T>>) -> Self {
        Self { mutable }
    }
}
