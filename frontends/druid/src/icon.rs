use druid::kurbo::BezPath;
use druid::{
    Affine, BoxConstraints, Color, Env, Event, EventCtx, KeyOrValue, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget,
};

use crate::theme;

// TODO: include and parse icons from assets folder at compile time
pub const ARTIST_ICON: SvgIcon = SvgIcon {
    path: "M11,14C12,14 13.05,14.16 14.2,14.44C13.39,15.31 13,16.33 13,17.5C13,18.39 13.25,19.23 13.78,20H3V18C3,16.81 3.91,15.85 5.74,15.12C7.57,14.38 9.33,14 11,14M11,12C9.92,12 9,11.61 8.18,10.83C7.38,10.05 7,9.11 7,8C7,6.92 7.38,6 8.18,5.18C9,4.38 9.92,4 11,4C12.11,4 13.05,4.38 13.83,5.18C14.61,6 15,6.92 15,8C15,9.11 14.61,10.05 13.83,10.83C13.05,11.61 12.11,12 11,12M18.5,10H20L22,10V12H20V17.5A2.5,2.5 0 0,1 17.5,20A2.5,2.5 0 0,1 15,17.5A2.5,2.5 0 0,1 17.5,15C17.86,15 18.19,15.07 18.5,15.21V10Z"
};

pub const ALBUM_ICON: SvgIcon = SvgIcon {
    path: "M12,11A1,1 0 0,0 11,12A1,1 0 0,0 12,13A1,1 0 0,0 13,12A1,1 0 0,0 12,11M12,16.5C9.5,16.5 7.5,14.5 7.5,12C7.5,9.5 9.5,7.5 12,7.5C14.5,7.5 16.5,9.5 16.5,12C16.5,14.5 14.5,16.5 12,16.5M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10,10 0 0,0 12,2Z"
};

pub const PLAYLIST_ICON: SvgIcon = SvgIcon {
    path: "M15,6H3V8H15V6M15,10H3V12H15V10M3,16H11V14H3V16M17,6V14.18C16.69,14.07 16.35,14 16,14A3,3 0 0,0 13,17A3,3 0 0,0 16,20A3,3 0 0,0 19,17V8H22V6H17Z"
};

pub const TRACK_ICON: SvgIcon = SvgIcon {
    path: "M12 3V13.55C11.41 13.21 10.73 13 10 13C7.79 13 6 14.79 6 17S7.79 21 10 21 14 19.21 14 17V7H18V3H12Z"
};

pub const NEXT_ICON: SvgIcon = SvgIcon {
    path: "M16,18H18V6H16M6,18L14.5,12L6,6V18Z",
};

pub const PREV_ICON: SvgIcon = SvgIcon {
    path: "M6,18V6H8V18H6M9.5,12L18,6V18L9.5,12Z",
};

pub const PLAY_ICON: SvgIcon = SvgIcon {
    path: "M8,5.14V19.14L19,12.14L8,5.14Z",
};

pub struct SvgIcon {
    path: &'static str,
}

pub struct Icon {
    path: BezPath,
    size: KeyOrValue<f64>,
    color: KeyOrValue<Color>,
}

impl Icon {
    pub fn new(icon: SvgIcon) -> Self {
        Icon {
            path: BezPath::from_svg(icon.path).unwrap(),
            size: 24.0.into(),
            color: theme::ICON_COLOR.into(),
        }
    }

    pub fn with_color(mut self, color: impl Into<KeyOrValue<Color>>) -> Self {
        self.set_color(color);
        self
    }

    pub fn set_color(&mut self, color: impl Into<KeyOrValue<Color>>) {
        self.color = color.into();
    }

    pub fn with_size(mut self, size: impl Into<KeyOrValue<f64>>) -> Self {
        self.set_size(size);
        self
    }

    pub fn set_size(&mut self, size: impl Into<KeyOrValue<f64>>) {
        self.size = size.into();
    }
}

impl<T> Widget<T> for Icon {
    fn event(&mut self, _ctx: &mut EventCtx, _ev: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _ev: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        let size = self.size.resolve(env);
        bc.constrain(Size::new(size, size))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let color = self.color.resolve(env);
        let size = self.size.resolve(env);
        ctx.transform(Affine::scale(size / 24.));
        ctx.with_save(|ctx| ctx.fill(&self.path, &color));
    }
}
