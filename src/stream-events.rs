// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::default();
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nos.lol").await?;

    client.connect().await;

    // Stream events from all connected relays
    let filter = Filter::new()
        .kinds([Kind::TextNote, Kind::LongFormTextNote])
        .limit(100);
    let mut stream = client
        .stream_events(filter, Duration::from_secs(15))
        .await?;

    while let Some(event) = stream.next().await {
        let kind_str = match event.kind {
            Kind::TextNote => "Text Note",
            Kind::LongFormTextNote => "Long-form Article",
            _ => "Unknown",
        };
        println!("Kind: {}", kind_str);
        println!("{}", event.as_json());
        println!("-------------------");
    }

    Ok(())
}
