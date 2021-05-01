#!/bin/sh
set -e
VERSION=$(egrep -o -m 1 "[0-9]+\.[0-9]+\.[0-9]+" ./Cargo.toml)
echo "Building monitoring-app image with version ${VERSION}"
docker build -t jsahm/monitoring-app:${VERSION} .
echo "Pushing image to hub"
docker push jsahm/monitoring-app:${VERSION}