#!/usr/bin/env bash

QDRANT_VOLUME_PATH="$HOME/cs/.cache/qdrant"

docker run \
    -tid \
    -p 6333:6333 \
    -p 6334:6334 \
    -v "$QDRANT_VOLUME_PATH:/qdrant/storage:z" \
    qdrant/qdrant
