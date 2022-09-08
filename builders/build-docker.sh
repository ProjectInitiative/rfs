#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
mkdir -p $SCRIPT_DIR/../target
docker build --no-cache $SCRIPT_DIR -t weed-build
# docker build $SCRIPT_DIR -t weed-build
docker run --rm --entrypoint cat weed-build  /go/bin/weed > $SCRIPT_DIR/../target/weed 
chmod +x $SCRIPT_DIR/../target/weed
tar -C $SCRIPT_DIR/../target -czvf $SCRIPT_DIR/../target/weed.tar.gz weed 
