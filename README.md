# Semantics

## Ideas:
* Given a wiki data of the 1M sentences, we index them in qudrant vector database
* We query a work or sentense, and vector database return the most similar related sentenses
* this is a cli application... based on the command the system works accordingly
* basically we have two modes, index and search --index ./path of the txt file & --search query

### Indexing Phase:
* reading the txt file buffer in buffer
* create the vector embeddings of the sentenses
* save the embeddings and payload in the vector database

### Query Phase:
* find the embeddings of the sentense
* find consine similarity with this embedding in the database
* return the top n results

### Dependencies:
* Qudrant Vector Database
* FastEmbed-rs ONNX Runtime

