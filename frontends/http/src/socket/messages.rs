use actix::Recipient;

use viewmodels::TrackModel;

#[derive(Message, Clone, Debug, Serialize)]
#[rtype(result = "()")]
#[serde(untagged)]
pub enum Message {
    PlayerMessage(PlayerMessage),
}

#[derive(Clone, Debug, Serialize)]
pub struct PlayerMessage {
    pub player_cursor: String,
    #[serde(flatten)]
    pub message: PlayerMessageData
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerMessageData {
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
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping;
