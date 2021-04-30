#!/bin/sh
set -e
VERSION=$(egrep -o -m 1 "[0-9]+\.[0-9]+\.[0-9]+" ./Cargo.toml)
echo "Building probes-app image with version ${VERSION}"
docker build -t jsahm/probes-app:${VERSION} .
echo "Pushing image to hub"
docker push jsahm/probes-app:${VERSION}