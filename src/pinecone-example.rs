use pinecone_sdk::models::{Cloud, DeletionProtection, IndexModel, Metric, Vector, WaitPolicy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Pinecone client
    // Note: Make sure to set PINECONE_API_KEY environment variable
    let client =
        pinecone_sdk::pinecone::default_client().expect("Failed to create Pinecone instance");

    // Create a new serverless index
    let index_name = "test-index";
    let dimension = 4; // Vector dimension size

    println!("Creating index '{}'...", index_name);

    let index: IndexModel = client
        .create_serverless_index(
            index_name,
            dimension,
            Metric::Cosine,
            Cloud::Aws,
            "us-east-1",
            DeletionProtection::Disabled,
            WaitPolicy::NoWait,
        )
        .await?;

    println!("Index created successfully!");

    // Wait a moment for the index to be ready
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

    // Get the index description to get the host
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
