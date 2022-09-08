#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
mkdir -p $SCRIPT_DIR/../target
# docker build --no-cache -t rfs-builder -f ./builders/Dockerfile $(dirname $SCRIPT_DIR) 
docker build -t rfs-builder -f ./builders/Dockerfile $(dirname $SCRIPT_DIR) 
docker run --rm --entrypoint cat rfs-builder  /bin/weed > $SCRIPT_DIR/../target/weed 
docker run --rm --entrypoint cat rfs-builder  /bin/rfs > $SCRIPT_DIR/../target/rfs 
chmod +x $SCRIPT_DIR/../target/weed
chmod +x $SCRIPT_DIR/../target/rfs


tar -C $SCRIPT_DIR/../target -czvf $SCRIPT_DIR/../target/weed.tar.gz weed 
