use qdrant_client::qdrant::SearchPointsBuilder;

use crate::core::{WIKI_COLLECTION_NAME, init_embedder, init_qdrant};

#[tokio::main]
pub async fn search(query: &str, limit: u64) {
    println!("Searching for: {}", query);

    let mut embedder = init_embedder();
    let embeddings = embedder
        .embed(vec![query], None)
        .unwrap_or_else(|e| panic!("Error while generating embedding for query: {}", e));
    let query_vector = embeddings
        .into_iter()
        .next()
        .expect("The sentence should successfully convert into one vector embedding");

    let qdrant = init_qdrant();
    let search_result = qdrant
        .search_points(
            SearchPointsBuilder::new(WIKI_COLLECTION_NAME, query_vector, limit).with_payload(true),
        )
        .await
        .unwrap_or_else(|e| panic!("Failed to search Qdrant database: {}", e));

    println!("\nTop {} Results:", limit);
    println!("-------------------");
    for (index, scored_point) in search_result.result.into_iter().enumerate() {
        let default = String::from("Unknown Text");
        let text = scored_point
            .payload
            .get("text")
            .and_then(|t| t.as_str())
            .unwrap_or(&default);

        println!("{}. Score: {:.4} | {}", index + 1, scored_point.score, text);
    }
}
