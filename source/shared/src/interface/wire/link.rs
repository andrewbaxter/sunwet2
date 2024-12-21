use {
    crate::interface::triple::FileHash,
    chrono::{
        DateTime,
        Utc,
    },
    serde::{
        Deserialize,
        Serialize,
    },
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct PrepareAudio {
    pub cover: Option<FileHash>,
    pub audio: FileHash,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum PrepareMedia {
    Audio(PrepareAudio),
    Video(FileHash),
    Image(FileHash),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Prepare {
    pub album: String,
    pub artist: String,
    pub name: String,
    pub media: PrepareMedia,
    pub media_time: f64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum WsL2S {
    Ready(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum WsS2L {
    Prepare(Prepare),
    Play(DateTime<Utc>),
    Pause,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum WsC2S {
    Prepare(Prepare),
    Ready(DateTime<Utc>),
    Pause,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum WsS2C {
    Play(DateTime<Utc>),
}
