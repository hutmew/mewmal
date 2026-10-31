use std::time::Duration;
use serde::Serialize;
use tokio::io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};
use mewmal_protocol::{ClientRequest, Response, DEFAULT_ADDR, TurnStartResult, ServerNotification, TurnDeltaParams, TurnCompletedParams};

const FIXED_REPLY: &[&str] = &["Hello", ", ", "this ", "is ", "mewmal ", "server", "!"];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let addr = std::env::var("MEWMAL_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    let listener = TcpListener::bind(&addr).await?;
    info!(%addr, "listening");

    loop {
        let (stream, peer) = listener.accept().await?;
        info!(%peer, "client connected");
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                warn!(%peer, error = %e, "connection error");
            }
            info!(%peer, "client disconnected");
        });
    }
}

async fn handle_connection(stream: TcpStream) -> anyhow::Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();
    let mut turn_seq: u64 = 0;

    while let Some(line) = lines.next_line().await? {
        info!(raw = %line, "recv");

        let req = match serde_json::from_str::<ClientRequest>(&line) {
            Ok(req) => req,
            Err(e) => {
                warn!(error = %e, "invalid request");
                continue;
            }
        };

        match req {
            ClientRequest::TurnStart { id, params } => {
                info!(input = %params.input, "turn/start");
                turn_seq += 1;
                let turn_id = format!("turn-{}", turn_seq);

                send(&mut writer, &Response::ok(id, &TurnStartResult {
                    turn_id: turn_id.clone(),
                })?).await?;

                for chunk in FIXED_REPLY {
                    let n = ServerNotification::TurnDelta(TurnDeltaParams {
                        turn_id: turn_id.clone(),
                        delta: chunk.to_string(),
                    });
                    send(&mut writer, &n).await?;
                    tokio::time::sleep(Duration::from_millis(150)).await;
                }

                send(&mut writer, &ServerNotification::TurnCompleted(TurnCompletedParams { turn_id } )).await?;
            }
        }
    }
    Ok(())
}

async fn send<W, T>(w: &mut W, msg: &T) -> anyhow::Result<()>
where
    W: AsyncWrite + Unpin,
    T: Serialize,
{
    let mut buf = serde_json::to_vec(msg)?;
    buf.push(b'\n');
    w.write_all(&buf).await?;
    w.flush().await?;

    Ok(())
}
