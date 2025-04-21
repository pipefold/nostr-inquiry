use dotenv::dotenv;
use pinecone_sdk::models::Vector;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the API key from environment variables
    let api_key =
        env::var("PINECONE_API_KEY").expect("PINECONE_API_KEY must be set in environment");

    // Initialize the Pinecone client with the API key
    let config = pinecone_sdk::pinecone::PineconeClientConfig {
        api_key: Some(api_key),
        ..Default::default()
    };
    let client = config.client().expect("Failed to create Pinecone instance");

    // Get the index description to get the host
    let index_name = "test-index"; // Replace with your existing index name
    let index_description = client.describe_index(index_name).await?;

    // Connect to the index
    let mut index = client.index(&index_description.host).await?;

    // Create some test vectors
    let vectors = [
        Vector {
            id: "vec1".to_string(),
            values: vec![1.0, 2.0, 3.0, 4.0],
            sparse_values: None,
            metadata: None,
        },
        Vector {
            id: "vec2".to_string(),
            values: vec![2.0, 3.0, 4.0, 5.0],
            sparse_values: None,
            metadata: None,
        },
    ];

    // Upsert the vectors
    println!("Upserting vectors...");
    let namespace = "test-namespace";
    let upsert_response = index.upsert(&vectors, &namespace.into()).await?;
    println!("Upserted {} vectors", upsert_response.upserted_count);

    // Perform a query
    println!("Performing query...");
    let query_vector = vec![1.0, 2.0, 3.0, 4.0];
    let query_response = index
        .query_by_value(query_vector, None, 2, &namespace.into(), None, None, None)
        .await?;

    println!("Query results:");
    for match_result in query_response.matches {
        println!("ID: {}, Score: {}", match_result.id, match_result.score);
    }

    Ok(())
}
