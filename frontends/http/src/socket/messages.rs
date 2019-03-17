use actix::Recipient;

use viewmodels::TrackModel;

#[derive(Message, Clone, Debug, Serialize)]
#[serde(
tag = "type",
content = "payload",
rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum Message {
    PlayerStateChanged(bool),
    CurrentlyPlayingChanged(Option<TrackModel>),
    QueueUpdated(Vec<TrackModel>),
}

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message, Debug)]
pub struct Disconnect {
    pub id: String,
}
