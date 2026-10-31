use serde::{Deserialize, Serialize};

pub const DEFAULT_ADDR: &str = "127.0.0.1:4500";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    Number(u64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum ClientRequest {
    #[serde(rename = "turn/start")]
    TurnStart { id: RequestId, params: TurnStartParams },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnStartParams {
    pub input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnStartResult {
    pub turn_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerMessage {
    Response(Response),
    Notification(ServerNotification),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: RequestId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

impl Response {
    pub fn ok<T: Serialize>(id: RequestId, result: &T) -> serde_json::Result<Self> {
        Ok(Self { id, result: Some(serde_json::to_value(result)?), error: None })
    }

    pub fn err(id: RequestId, error: RpcError) -> Self {
        Self { id, result: None, error: Some(error) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum ServerNotification {
    #[serde(rename = "turn/delta")]
    TurnDelta(TurnDeltaParams),
    #[serde(rename = "turn/completed")]
    TurnCompleted(TurnCompletedParams),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnDeltaParams {
    pub turn_id: String,
    pub delta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnCompletedParams {
    pub turn_id: String,
}
