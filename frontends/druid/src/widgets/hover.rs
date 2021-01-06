use crate::theme;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, RenderContext, Size, UpdateCtx, Widget, WidgetPod,
};

pub struct Hover<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> Hover<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        Self {
            inner: WidgetPod::new(inner).boxed(),
        }
    }
}

impl<T: Data> Widget<T> for Hover<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let background = if ctx.is_hot() {
            env.get(theme::HOVER_HOT_COLOR)
        } else {
            env.get(theme::HOVER_COLD_COLOR)
        };
        let rounded_rect = ctx.size().to_rect().to_rounded_rect(4.0);
        self.inner.paint(ctx, data, env);
        ctx.fill(rounded_rect, &background);
    }
}

pub trait HoverExt<T: Data>: Widget<T> + Sized + 'static {
    fn hover(self) -> Hover<T> {
        Hover::new(self)
    }
}

impl<T: Data, W: Widget<T> + 'static> HoverExt<T> for W {}
