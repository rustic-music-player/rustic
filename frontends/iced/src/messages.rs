use crate::views::MainView;

#[derive(Debug, Clone)]
pub enum Message {
    OpenView(MainView),
    Search(String),
    ChangePlayer
}
