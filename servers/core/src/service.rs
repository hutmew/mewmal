use std::collections::HashSet;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use mewmal_protocol::types::{RunId, ServerEvent};

const EVENT_CAPACITY: usize = 64;
const MAX_RUNS_PER_SUBSCRIPTION: usize = 1024;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("The run already exists")]
    Duplicated,
    #[error("The run was not found")]
    NotFound,
    #[error("The run limit has been reached")]
    Limit,
    #[error("The subscription is closing")]
    Closed,
}

struct ActiveRun {
    id: RunId,
    cancel: CancellationToken,
}

#[derive(Default)]
struct State {
    active: Option<ActiveRun>,
    seen: HashSet<RunId>,
}

pub struct RunService {
    state: Mutex<State>,
    events: mpsc::Sender<ServerEvent>,
    lifetime: CancellationToken,
    tasks: TaskTracker
}

