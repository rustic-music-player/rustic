use druid::ExtEventSink;
use futures::future::BoxFuture;

pub trait StreamDelegate {
    fn observe(&self, sink: ExtEventSink) -> BoxFuture<'static, ()>;
}

pub trait BoxedStreamDelegate: StreamDelegate {
    fn boxed(self) -> Box<dyn StreamDelegate>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<T: ?Sized> BoxedStreamDelegate for T where T: StreamDelegate {}
