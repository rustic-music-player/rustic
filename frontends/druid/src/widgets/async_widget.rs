use crate::state::{AsyncData, AsyncDataState};
use druid::{widget::prelude::*, Data, Point, WidgetExt, WidgetPod};

pub struct Async<TResolved, TPending, TRejected> {
    pending_maker: Box<dyn Fn() -> Box<dyn Widget<TPending>>>,
    resolved_maker: Box<dyn Fn() -> Box<dyn Widget<TResolved>>>,
    rejected_maker: Box<dyn Fn() -> Box<dyn Widget<TRejected>>>,
    widget: AsyncWidget<TResolved, TPending, TRejected>,
}

#[allow(clippy::large_enum_variant)]
enum AsyncWidget<T, D, E> {
    Empty,
    Pending(WidgetPod<D, Box<dyn Widget<D>>>),
    Resolved(WidgetPod<T, Box<dyn Widget<T>>>),
    Rejected(WidgetPod<E, Box<dyn Widget<E>>>),
}

impl<D: Data, T: Data, E: Data> Async<T, D, E> {
    pub fn new<WD, WT, WE>(
        pending_maker: impl Fn() -> WD + 'static,
        resolved_maker: impl Fn() -> WT + 'static,
        rejected_maker: impl Fn() -> WE + 'static,
    ) -> Self
    where
        WD: Widget<D> + 'static,
        WT: Widget<T> + 'static,
        WE: Widget<E> + 'static,
    {
        Self {
            pending_maker: Box::new(move || pending_maker().boxed()),
            resolved_maker: Box::new(move || resolved_maker().boxed()),
            rejected_maker: Box::new(move || rejected_maker().boxed()),
            widget: AsyncWidget::Empty,
        }
    }

    fn rebuild_widget(&mut self, state: AsyncDataState) {
        self.widget = match state {
            AsyncDataState::Empty => AsyncWidget::Empty,
            AsyncDataState::Pending => AsyncWidget::Pending(WidgetPod::new((self.pending_maker)())),
            AsyncDataState::Resolved => {
                AsyncWidget::Resolved(WidgetPod::new((self.resolved_maker)()))
            }
            AsyncDataState::Rejected => {
                AsyncWidget::Rejected(WidgetPod::new((self.rejected_maker)()))
            }
        };
    }
}

impl<D: Data, T: Data, E: Data> Widget<AsyncData<T, D, E>> for Async<T, D, E> {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AsyncData<T, D, E>,
        env: &Env,
    ) {
        if data.state() == self.widget.state() {
            match data {
                AsyncData::Empty => {}
                AsyncData::Pending(d) => {
                    self.widget.with_pending(|w| w.event(ctx, event, d, env));
                }
                AsyncData::Resolved(o) => {
                    self.widget.with_resolved(|w| w.event(ctx, event, o, env));
                }
                AsyncData::Rejected(e) => {
                    self.widget.with_rejected(|w| w.event(ctx, event, e, env));
                }
            };
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AsyncData<T, D, E>,
        env: &Env,
    ) {
        if data.state() != self.widget.state() {
            // possible if getting lifecycle after an event that changed the data,
            // or on WidgetAdded
            self.rebuild_widget(data.state());
        }
        assert_eq!(data.state(), self.widget.state(), "{:?}", event);
        match data {
            AsyncData::Empty => {}
            AsyncData::Pending(pending_state) => {
                self.widget
                    .with_pending(|w| w.lifecycle(ctx, event, pending_state, env));
            }
            AsyncData::Resolved(resolved_state) => {
                self.widget
                    .with_resolved(|w| w.lifecycle(ctx, event, resolved_state, env));
            }
            AsyncData::Rejected(rejected_state) => {
                self.widget
                    .with_rejected(|w| w.lifecycle(ctx, event, rejected_state, env));
            }
        };
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &AsyncData<T, D, E>,
        data: &AsyncData<T, D, E>,
        env: &Env,
    ) {
        if old_data.state() != data.state() {
            self.rebuild_widget(data.state());
            ctx.children_changed();
        } else {
            match data {
                AsyncData::Empty => {}
                AsyncData::Pending(pending_state) => {
                    self.widget
                        .with_pending(|w| w.update(ctx, pending_state, env));
                }
                AsyncData::Resolved(resolved_state) => {
                    self.widget
                        .with_resolved(|w| w.update(ctx, resolved_state, env));
                }
                AsyncData::Rejected(rejected_state) => {
                    self.widget
                        .with_rejected(|w| w.update(ctx, rejected_state, env));
                }
            };
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AsyncData<T, D, E>,
        env: &Env,
    ) -> Size {
        match data {
            AsyncData::Empty => None,
            AsyncData::Pending(d) => self.widget.with_pending(|w| {
                let size = w.layout(ctx, bc, d, env);
                w.set_origin(ctx, d, env, Point::ORIGIN);
                size
            }),
            AsyncData::Resolved(o) => self.widget.with_resolved(|w| {
                let size = w.layout(ctx, bc, o, env);
                w.set_origin(ctx, o, env, Point::ORIGIN);
                size
            }),
            AsyncData::Rejected(e) => self.widget.with_rejected(|w| {
                let size = w.layout(ctx, bc, e, env);
                w.set_origin(ctx, e, env, Point::ORIGIN);
                size
            }),
        }
        .unwrap_or_default()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AsyncData<T, D, E>, env: &Env) {
        match data {
            AsyncData::Empty => {}
            AsyncData::Pending(d) => {
                self.widget.with_pending(|w| w.paint(ctx, d, env));
            }
            AsyncData::Resolved(o) => {
                self.widget.with_resolved(|w| w.paint(ctx, o, env));
            }
            AsyncData::Rejected(e) => {
                self.widget.with_rejected(|w| w.paint(ctx, e, env));
            }
        };
    }
}

impl<T, D, E> AsyncWidget<T, D, E> {
    fn state(&self) -> AsyncDataState {
        match self {
            Self::Empty => AsyncDataState::Empty,
            Self::Pending(_) => AsyncDataState::Pending,
            Self::Resolved(_) => AsyncDataState::Resolved,
            Self::Rejected(_) => AsyncDataState::Rejected,
        }
    }

    fn with_pending<R, F: FnOnce(&mut WidgetPod<D, Box<dyn Widget<D>>>) -> R>(
        &mut self,
        f: F,
    ) -> Option<R> {
        if let Self::Pending(widget) = self {
            Some(f(widget))
        } else {
            None
        }
    }

    fn with_resolved<R, F: FnOnce(&mut WidgetPod<T, Box<dyn Widget<T>>>) -> R>(
        &mut self,
        f: F,
    ) -> Option<R> {
        if let Self::Resolved(widget) = self {
            Some(f(widget))
        } else {
            None
        }
    }

    fn with_rejected<R, F: FnOnce(&mut WidgetPod<E, Box<dyn Widget<E>>>) -> R>(
        &mut self,
        f: F,
    ) -> Option<R> {
        if let Self::Rejected(widget) = self {
            Some(f(widget))
        } else {
            None
        }
    }
}
