use std::sync::Arc;
use iced::{Application, Command, Element, Settings, Column, Text, Length, HorizontalAlignment, button, Row, Scrollable, scrollable};
use rustic_core::Rustic;
use crate::messages::Message;
use crate::views::MainView;
use crate::component::Component;

mod messages;
mod component;
mod views;

pub fn start(app: Arc<Rustic>) {
    IcedApplication::run(Settings::with_flags(app));
}

struct IcedApplication {
    app: Arc<Rustic>,
    sidenav: Vec<(String, button::State, MainView)>,
    current_view: MainView,
    main_scroll: scrollable::State
}

impl Application for IcedApplication {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Arc<Rustic>;

    fn new(app: Self::Flags) -> (Self, Command<Self::Message>) {
        let sidenav = vec![
            ("Library".into(), button::State::new(), MainView::Library),
            ("Playlists".into(), button::State::new(), MainView::Playlists),
            ("Explore".into(), button::State::new(), MainView::Explore),
        ];
        (
            IcedApplication {
                app,
                sidenav,
                current_view: MainView::default(),
                main_scroll: scrollable::State::new()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        format!("Rustic")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::OpenView(view) => {
                self.current_view = view;
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut sidenav = Column::new()
            .max_width(300)
            .spacing(20);
        for (label, state, view) in self.sidenav.iter_mut() {
            let btn = button::Button::new(state, Text::new(label.clone()))
                .on_press(Message::OpenView(*view));
            sidenav = sidenav.push(btn);
        }
        let content = self.current_view.view(&self.app);
        let scroll_container = Scrollable::new(&mut self.main_scroll).push(content);

        Row::new()
            .push(sidenav)
            .push(scroll_container)
            .into()
    }
}
