use jsonrpsee::{
    core::{RpcResult, SubscriptionResult},
    proc_macros::rpc,
};

use crate::types::{
    CancelRunRequest, CancelRunResponse, ServerEvent, StartRunRequest, StartRunResponse,
    SystemInfoResponse,
};

#[rpc(client, server)]
pub trait CoreRpc {
    #[method(name = "system.info")]
    async fn system_info(&self) -> RpcResult<SystemInfoResponse>;
    #[method(
        name = "run.start",
        param_kind = map,
        with_extensions
    )]
    async fn start_run(&self, req: StartRunRequest) -> RpcResult<StartRunResponse>;
    #[method(
        name = "run.cancel",
        param_kind = map,
        with_extensions
    )]
    async fn cancel_run(&self, req: CancelRunRequest) -> RpcResult<CancelRunResponse>;
    #[subscription(
        name = "events.subscribe" => "events.event",
        unsubscribe = "events.unsubscribe",
        item = ServerEvent
    )]
    async fn subscribe_server_events(&self) -> SubscriptionResult;
}
