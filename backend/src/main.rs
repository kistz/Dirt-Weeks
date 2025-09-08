use gbxremote2::{ClientError, ServerClient, types::WayPointEvent};
use serde::{Deserialize, Serialize};

use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut server = ServerClient::new("127.0.0.1:5001").await;

    let _: Result<bool, ClientError> = server.call("SetApiVersion", "2023-03-24").await;

    let _: Result<bool, ClientError> = server
        .call("Authenticate", ("SuperAdmin", "SuperAdmin"))
        .await;

    let _: Result<bool, ClientError> = server.call("EnableCallbacks", true).await;

    let _: Result<bool, ClientError> = server
        .call(
            "TriggerModeScriptEventArray",
            ("XmlRpc.EnableCallbacks", ["true"]),
        )
        .await;

    let _: Result<bool, ClientError> = server
        .call("ChatSendServerMessage", "Hey from Rust owo")
        .await;

    server.subscribe("Trackmania.Event.WayPoint", |json| {
        let nicely_typed = serde_json::from_str::<WayPointEvent>(json).unwrap();

        println!("This is nice: {nicely_typed:#?}");
    });

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("CTRL + C: Closing the application 👋");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    Ok(())
}
