use crate::state::State;
use druid::{Command, Env, ExtEventSink, Target};
use futures::future::BoxFuture;

pub trait PartialDelegate {
    fn command(
        &mut self,
        sink: ExtEventSink,
        target: Target,
        cmd: &Command,
        data: &mut State,
        env: &Env,
    ) -> Option<BoxFuture<'static, ()>>;
}

pub trait BoxedDelegate: PartialDelegate {
    fn boxed(self) -> Box<dyn PartialDelegate>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

impl<T: ?Sized> BoxedDelegate for T where T: PartialDelegate {}
