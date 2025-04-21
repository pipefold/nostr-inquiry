use nostr_sdk::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a new client with default settings
    let client = Client::default();

    // Add some popular relays
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://relay.nostr.band").await?;
    client.add_relay("wss://nos.lol").await?;

    // Connect to the relays
    client.connect().await;

    // Create a filter for the latest 10 text notes (Kind 1)
    let filter = Filter::new()
        .kind(Kind::TextNote) // Kind 1 - regular text notes
        .limit(10); // Get only 10 notes

    // Fetch events from all connected relays with a 10 second timeout
    let events = client.fetch_events(filter, Duration::from_secs(10)).await?;

    // Print the events
    for event in events {
        println!("Author: {}", event.pubkey);
        println!("Content: {}", event.content);
        println!("Created at: {}", event.created_at);
        println!("-------------------");
    }

    Ok(())
}
