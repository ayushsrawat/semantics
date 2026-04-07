use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    time::Instant,
};

use fastembed::{InitOptions, TextEmbedding};

const FASTEMBED_EMBED_MODEL: fastembed::EmbeddingModel = fastembed::EmbeddingModel::BGESmallENV15;
const FASTEMBED_CACHE_DIR: &str = "/Users/ayushrawat/.cache/fastembed";
const DOCS_BATCH_SIZE: usize = 32;

pub fn index(path: &str) {
    let path = Path::new(path);
    if !path.exists() || !path.is_file() {
        panic!("Error: Invalid input source - {:?}", path.as_os_str());
    }
    println!("Indexing path: {:?}", path.as_os_str());

    let start = Instant::now();
    let file = File::open(path).unwrap_or_else(|e| panic!("Error opening file: {}", e));
    let reader: BufReader<File> = BufReader::new(file);

    let mut model = TextEmbedding::try_new(
        InitOptions::new(FASTEMBED_EMBED_MODEL)
            .with_show_download_progress(true)
            .with_cache_dir(PathBuf::from(FASTEMBED_CACHE_DIR)),
    )
    .unwrap_or_else(|e| panic!("Error while create embedding model: {}", e));

    let mut batch: Vec<String> = Vec::with_capacity(DOCS_BATCH_SIZE);

    // todo!(index line in the vector database)
    // step 1: generate vector embeddings of the line
    // step 2: save the embeddings and payload in the qudrant database
    // handle cases when data has been already indexed...
    // don't need to do it again, don't need to generate the embeddings

    // here I would like to create batches of these lines and ask for the model to create the embeddings on per batch
    let mut batch_embed_start = Instant::now();
    for line in reader.lines() {
        let line = line.unwrap_or_else(|e| panic!("Error reading line: {}", e));
        batch.push(line);

        if batch.len() >= DOCS_BATCH_SIZE {
            let embeddings = generate_embeddings(&mut model, &batch);
            println!("Embeddings length: {}", embeddings.len());
            println!("Embeddings elapsed {:?}", batch_embed_start.elapsed());
            batch_embed_start = Instant::now();
            batch.clear();
        }
    }
    if !batch.is_empty() {
        let embeddings = generate_embeddings(&mut model, &batch);
        println!("Last Embeddings length: {}", embeddings.len());
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
