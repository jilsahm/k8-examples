#!/bin/sh
set -e
VERSION=$(egrep -o -m 1 "[0-9]+\.[0-9]+\.[0-9]+" ./Cargo.toml)
echo "Building redis-app image with version ${VERSION}"
docker build -t jsahm/redis-app:${VERSION} .
echo "Pushing image to hub"
docker push jsahm/redis-app:${VERSION}