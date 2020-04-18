use crate::views::MainView;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenView(MainView)
}
