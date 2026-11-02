use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Mewmal's protocol version
pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RunId(Uuid);

impl RunId {
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub const fn uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    protocol_version: u32,
    server_version: String,
}

impl SystemInfoResponse {
    pub fn new(protocol_version: u32, server_version: String) -> Self {
        Self {
            protocol_version,
            server_version,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartRunRequest {
    run_id: RunId,
    input: String,
}

impl StartRunRequest {
    pub fn input(&self) -> &str {
        &self.input
    }
    
    pub fn run_id(&self) -> RunId {
        self.run_id
    }
}

// 実行の受付を表す。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartRunResponse {
    run_id: RunId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRunRequest {
    run_id: RunId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRunResponse {
    accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerEvent {
    run_id: RunId,
    // 実行ごとの連番。Startedを0として増加させる
    seq: u64,
    #[serde(flatten)]
    event: ServerEventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEventKind {
    Started,
    OutputDelta { text: String },
    Completed,
    Failed { code: String, message: String },
    Cancelled,
}
