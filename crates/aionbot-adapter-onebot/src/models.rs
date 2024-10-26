use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sender {
    pub user_id: Option<i64>,
    pub nickname: Option<String>,
    pub sex: Option<String>,
    pub age: Option<i32>,
    pub area: Option<String>,
    pub level: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Anonymous {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageData {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageSegment {
    pub r#type: String,
    pub data: MessageData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
    pub message_type: String,
    pub sub_type: String,
    pub message_id: i64,
    pub group_id: Option<i64>,
    pub user_id: i64,
    pub anonymous: Option<String>,
    pub message: Vec<MessageSegment>,
    pub message_format: String,
    pub raw_message: String,
    pub font: i32,
    pub sender: Sender,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionParams {
    pub group_id: Option<i64>,
    pub user_id: Option<i64>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Action {
    pub action: String,
    pub params: ActionParams,
    pub echo: Option<String>,
}
