use crate::state::State;
pub use druid::theme::*;
use druid::{Color, Env, FontDescriptor, FontFamily, FontWeight, Key};

pub fn grid(multiplier: f64) -> f64 {
    GRID * multiplier
}

const GRID: f64 = 8.0;

const GREY_200: Color = Color::rgb8(245, 245, 245);
const GREY_300: Color = Color::rgb8(224, 224, 224);
const GREY_900: Color = Color::rgb8(33, 33, 33);

const BORDER_DARK_COLOR: Color = Color::rgba8(0, 0, 0, 222);
const BORDER_LIGHT_COLOR: Color = Color::rgba8(0, 0, 0, 25);

// approx 0.87
pub const TEXT_COLOR_ACTIVE: Color = Color::rgba8(0, 0, 0, 222);
// approx 0.54
pub const TEXT_COLOR_INACTIVE: Color = Color::rgba8(0, 0, 0, 138);

pub const ICON_COLOR: Key<Color> = Key::new("app.icon-color");

pub const DEFAULT_FONT_SIZE: f64 = 13.0;

const DEFAULT_FONT: FontDescriptor =
    FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(DEFAULT_FONT_SIZE);

pub const UI_FONT_MEDIUM: Key<FontDescriptor> = Key::new("app.font-medium");

pub const HOVER_HOT_COLOR: Key<Color> = Key::new("app.hover-hot-color");
pub const HOVER_COLD_COLOR: Key<Color> = Key::new("app.hover-cold-color");

pub fn setup(env: &mut Env, _: &State) {
    env.set(WINDOW_BACKGROUND_COLOR, Color::WHITE);
    env.set(BACKGROUND_DARK, GREY_300);
    env.set(BACKGROUND_LIGHT, GREY_200);
    env.set(BORDER_DARK, BORDER_DARK_COLOR);
    env.set(BORDER_LIGHT, BORDER_LIGHT_COLOR);

    env.set(LABEL_COLOR, TEXT_COLOR_ACTIVE);
    env.set(ICON_COLOR, TEXT_COLOR_INACTIVE);

    env.set(UI_FONT, DEFAULT_FONT);
    env.set(UI_FONT_BOLD, DEFAULT_FONT.with_weight(FontWeight::BOLD));
    env.set(UI_FONT_MEDIUM, DEFAULT_FONT.with_weight(FontWeight::MEDIUM));

    env.set(HOVER_HOT_COLOR, Color::rgba(0.0, 0.0, 0.0, 0.05));
    env.set(HOVER_COLD_COLOR, Color::rgba(0.0, 0.0, 0.0, 0.0));
}
