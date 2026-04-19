use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use fastembed::TextEmbedding;
use qdrant_client::{
    Qdrant,
    qdrant::{
        CollectionExistsRequest, CreateCollectionBuilder, Distance, PointStruct,
        UpsertPointsBuilder, Value, VectorParamsBuilder,
    },
};
use uuid::Uuid;

use crate::core::{
    FASTEMBED_EMBED_MODEL, WIKI_COLLECTION_NAME, get_model_dimension, init_embedder, init_qdrant,
};

const DOCS_BATCH_SIZE: usize = 256;
const INPUT_SENTENSES_LIMIT: usize = 10_000; // todo: consider taking limit as a cmd option

#[tokio::main]
pub async fn index(path: &str) {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        panic!("Error: Invalid input source - {:?}", path.as_os_str());
    }
    println!("Indexing path: {:?}", path.as_os_str());

    let start = Instant::now();
    let file = File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e));
    let reader: BufReader<File> = BufReader::new(file);

    let mut embedder = init_embedder();
    let model_dim: usize = get_model_dimension();
    println!(
        "Dimension of model {} is [{}]",
        FASTEMBED_EMBED_MODEL, model_dim
    );
    let qdrant = init_qdrant();

    let response = qdrant
        .list_collections()
        .await
        .unwrap_or_else(|e| panic!("Failed to list collections: {}", e));
    println!("Available Collections : {:#?}", response);

    let collections_already_exists: bool = qdrant
        .collection_exists(CollectionExistsRequest::from(WIKI_COLLECTION_NAME))
        .await
        .unwrap_or_else(|e| panic!("Error while checking if collection exists: {}", e));

    if !collections_already_exists {
        println!("Creating Qdrant Collction : {}", WIKI_COLLECTION_NAME);
        let response = qdrant
            .create_collection(
                CreateCollectionBuilder::new(WIKI_COLLECTION_NAME)
                    .vectors_config(VectorParamsBuilder::new(model_dim as u64, Distance::Cosine)),
            )
            .await
            .unwrap_or_else(|e| panic!("Error create new collection: {}", e));
        println!(
            "Successfully created new qdrant collection in : {}",
            response.time
        )
    } else {
        println!(
            "Qdrant collection {} already exists! ",
            WIKI_COLLECTION_NAME
        );
    }

    let mut batch: Vec<String> = Vec::with_capacity(DOCS_BATCH_SIZE);
    let mut batch_embed_start = Instant::now();
    let mut sentense_counter: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| panic!("Error reading line: {}", e));
        batch.push(line);
        sentense_counter += 1;
        if sentense_counter >= INPUT_SENTENSES_LIMIT as i32 {
            println!(
                "Sentences count LIMIT crossed!! Read {} sentenses, stopping!!",
                sentense_counter
            );
            break;
        }

        if batch.len() >= DOCS_BATCH_SIZE {
            let embeddings = generate_embeddings(&mut embedder, &batch);
            upsert_embeddings(&batch, embeddings, &qdrant).await;

            println!(
                "Successfully inserted batch! Elapsed {:?}",
                batch_embed_start.elapsed()
            );
            batch_embed_start = Instant::now();
            batch.clear();
        }
    }

    // todo! just calculating the embeddings of 256 sentences takes on average 5 secs
    // for 1M senteses it takes 5.42 hrs which is quite a calculation...
    if !batch.is_empty() {
        let embeddings = generate_embeddings(&mut embedder, &batch);
        upsert_embeddings(&batch, embeddings, &qdrant).await;
    }

    println!("Time elapsed while reading file: {:?}", start.elapsed());
}

fn generate_embeddings(model: &mut TextEmbedding, batch: &Vec<String>) -> Vec<Vec<f32>> {
    let embeddings = model.embed(batch, Some(DOCS_BATCH_SIZE));
    match embeddings {
        Ok(v) => v,
        Err(e) => panic!("Error while generating embeddings: {}", e),
    }
}

async fn upsert_embeddings(batch: &Vec<String>, embeddings: Vec<Vec<f32>>, qdrant: &Qdrant) {
    let mut points = Vec::with_capacity(batch.len());
    let namespace = Uuid::NAMESPACE_OID;

    for (text, embedding) in batch.iter().zip(embeddings.into_iter()) {
        let point_id = Uuid::new_v5(&namespace, text.as_bytes()).to_string();

        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert("text".to_string(), text.clone().into());

        let point = PointStruct::new(point_id, embedding, payload);
        points.push(point);
    }

    qdrant
        .upsert_points(UpsertPointsBuilder::new(WIKI_COLLECTION_NAME, points))
        .await
        .unwrap_or_else(|e| panic!("Failed to insert batch into Qdrant: {}", e));
}
