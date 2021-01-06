use crate::icon::{Icon, PLAYLIST_ICON};
use crate::state::{Link, Route, SidebarEntry, State};
use crate::widgets::hover::HoverExt;
use crate::{commands, theme};
use druid::widget::{Flex, Label, LineBreaking, List, Scroll, ViewSwitcher};
use druid::{Widget, WidgetExt};

pub fn make_sidebar() -> impl Widget<State> {
    let menu = List::new(|| {
        ViewSwitcher::<SidebarEntry, SidebarEntry>::new(
            |state, _| state.clone(),
            |entry, _, _| match entry {
                SidebarEntry::Header(label) => Label::new(label.as_str())
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .with_font(theme::UI_FONT_BOLD)
                    .with_text_color(theme::TEXT_COLOR_INACTIVE)
                    .padding((theme::grid(2.), theme::grid(1.)))
                    .expand_width()
                    .boxed(),
                SidebarEntry::Playlist(playlist) => {
                    let route = Route::PlaylistDetails(Link::from(playlist));
                    let label = Label::new(playlist.title.as_str())
                        .with_line_break_mode(LineBreaking::WordWrap);

                    Flex::row()
                        .with_child(Icon::new(PLAYLIST_ICON))
                        .with_spacer(theme::grid(1.))
                        .with_flex_child(label, 1.)
                        .padding((theme::grid(2.), theme::grid(1.)))
                        .expand_width()
                        .hover()
                        .on_click(move |ctx, _, _| {
                            ctx.submit_command(commands::NAVIGATE.with(route.clone()))
                        })
                        .boxed()
                }
                SidebarEntry::NavEntry(entry, route, icon) => {
                    let route = route.clone();
                    let label =
                        Label::new(entry.as_str()).with_line_break_mode(LineBreaking::WordWrap);

                    Flex::row()
                        .with_child(Icon::new(icon.into()))
                        .with_spacer(theme::grid(1.))
                        .with_flex_child(label, 1.)
                        .padding((theme::grid(2.), theme::grid(1.)))
                        .expand_width()
                        .hover()
                        .on_click(move |ctx, _, _| {
                            ctx.submit_command(commands::NAVIGATE.with(route.clone()))
                        })
                        .boxed()
                }
            },
        )
    })
    .background(theme::BACKGROUND_LIGHT);
    Scroll::new(menu).vertical().lens(State::sidebar)
}
