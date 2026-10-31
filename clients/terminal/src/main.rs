use std::io::Write;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use mewmal_protocol::{ClientRequest, RequestId, DEFAULT_ADDR, TurnStartParams, ServerMessage, ServerNotification};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = std::env::var("MEWMAL_ADDR").unwrap_or_else(|_| DEFAULT_ADDR.to_string());
    let stream = TcpStream::connect(&addr).await?;
    println!("connected to {addr}");

    let (reader, mut writer) = stream.into_split();
    let mut server_lines = BufReader::new(reader).lines();
    let mut stdin_lines = BufReader::new(tokio::io::stdin()).lines();
    let mut next_id = 1;

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let Some(input) = stdin_lines.next_line().await? else { break };
        if input.trim().is_empty() {
            continue;
        }

        let req = ClientRequest::TurnStart {
            id: RequestId::Number(next_id),
            params: TurnStartParams { input },
        };
        next_id += 1;

        let mut buf = serde_json::to_vec(&req)?;
        buf.push(b'\n');
        writer.write_all(&buf).await?;

        // turn/completed が届くまで受信して表示する
        loop {
            let Some(line) = server_lines.next_line().await? else {
                println!("\nserver closed the connection");
                return Ok(());
            };
            match serde_json::from_str::<ServerMessage>(&line)? {
                ServerMessage::Response(res) => {
                    if let Some(err) = res.error {
                        eprintln!("error {}: {}", err.code, err.message);
                        break;
                    }
                }
                ServerMessage::Notification(ServerNotification::TurnDelta(p)) => {
                    print!("{}", p.delta);
                    std::io::stdout().flush()?;
                }
                ServerMessage::Notification(ServerNotification::TurnCompleted(_)) => {
                    println!();
                    break;
                }
            }
        }
    }
    Ok(())
}
