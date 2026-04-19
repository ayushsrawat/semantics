# Semantics

A lightweight Command-Line Interface (CLI) application for Semantic Search.

Unlike traditional keyword search that looks for exact token matches, **Semantic Search** understands the contextual meaning and intent behind a query. By converting text into high-dimensional vector embeddings, the system can find sentences that are conceptually similar to what you are looking for, even if they share zero overlapping words.

## Quick Start

### 1. Indexing

Read your large corpus of sentences, convert them to mathematical vector embeddings locally, and store them in the Qdrant database.

```bash
./semantics --index ./data/wiki-sentences.txt
```

### 2. Searching

Query your database using natural language. The system evaluates the Cosine Similarity between your query's vector and the database vectors to return the top most relevant results.

```bash
./semantics --search "How did the universe begin?"

./semantics --search "Machine learning algorithms" --top 10
```

## How It Works

1. **Text Parsing**: Streams a large `txt` file line-by-line efficiently using buffered readers.
2. **Local Embeddings**: Generates representations of sentences completely locally using `fastembed` (BGESmallENV15 model) to preserve privacy and reduce latency.
3. **Vector Storage**: Upserts the generated arrays into **Qdrant** configured manually for Cosine Distance metrics.
4. **Vector Search**: Upon querying, calculates the distance between vectors and retrieves the nearest neighbors.

## Dependencies

This app is heavily powered by the following Rust crates:

- [`fastembed`](https://crates.io/crates/fastembed) **(v5.13)**: For lightning-fast, local text generation (powered by the ONNX runtime).
- [`qdrant-client`](https://crates.io/crates/qdrant-client) **(v1.17)**: The official async client to interface with our Qdrant instance.
- [`tokio`](https://crates.io/crates/tokio) **(v1.52)**: The blazing-fast asynchronous runtime enabling non-blocking database queries.
- [`uuid`](https://crates.io/crates/uuid) **(v1)**: Deterministic UUIDv5 generation based on text to avoid duplicated vectors in the database.
