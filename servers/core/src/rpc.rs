use jsonrpsee::core::RpcResult;
use jsonrpsee::Extensions;
use jsonrpsee::server::middleware::rpc::RpcService;
use mewmal_protocol::CoreRpcServer;
use mewmal_protocol::types::{SystemInfoResponse, PROTOCOL_VERSION, StartRunRequest, StartRunResponse};

impl CoreRpcServer for RpcService {
    async fn system_info(&self) -> RpcResult<SystemInfoResponse> {
        Ok(SystemInfoResponse::new(PROTOCOL_VERSION, env!("CARGO_PKG_VERSION").to_owned()))
    }

    async fn start_run(&self, ext: &Extensions, req: StartRunRequest) -> RpcResult<StartRunResponse> {
        unimplemented!();
    }
}