#!/bin/bash

set -euo pipefail

GRAPL_LOCAL_REGISTRY_PORT=5000
GRAPL_LOCAL_REGISTRY_NAME="grapl_local_registry"

if [[ -n "$(docker ps --quiet --filter name="${GRAPL_LOCAL_REGISTRY_NAME}")" ]]; then
    echo "already running."
else
    echo "Creating container registry."
    docker run --detach \
        --rm \
        --publish "127.0.0.1:${GRAPL_LOCAL_REGISTRY_PORT}:${GRAPL_LOCAL_REGISTRY_PORT}" \
        --name="${GRAPL_LOCAL_REGISTRY_NAME}" \
        registry:2.7
fi
