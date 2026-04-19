use std::path::PathBuf;

use fastembed::{InitOptions, TextEmbedding};
use qdrant_client::Qdrant;

pub const FASTEMBED_EMBED_MODEL: fastembed::EmbeddingModel =
    fastembed::EmbeddingModel::BGESmallENV15;
pub const FASTEMBED_CACHE_DIR: &str = "/Users/ayushrawat/.cache/fastembed";
pub const QDRANT_URL: &str = "http://localhost:6334";
pub const WIKI_COLLECTION_NAME: &str = "wiki-sentences";

pub fn init_embedder() -> TextEmbedding {
    TextEmbedding::try_new(
        InitOptions::new(FASTEMBED_EMBED_MODEL)
            .with_show_download_progress(true)
            .with_cache_dir(PathBuf::from(FASTEMBED_CACHE_DIR)),
    )
    .unwrap_or_else(|e| panic!("Error while create embedding model: {}", e))
}

pub fn init_qdrant() -> Qdrant {
    Qdrant::from_url(QDRANT_URL)
        .build()
        .unwrap_or_else(|e| panic!("Error while connecting to qudrant client: {}", e))
}

pub fn get_model_dimension() -> usize {
    let supported_models = TextEmbedding::list_supported_models();
    let model_info = supported_models
        .iter()
        .find(|info| info.model == FASTEMBED_EMBED_MODEL)
        .expect("Model info should exist for supported models");

    model_info.dim
}
