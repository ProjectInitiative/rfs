FROM docker.io/golang:1.18 as build-swfs 


RUN git clone --depth 1 -b master https://github.com/chrislusf/seaweedfs /go/src/seaweedfs
WORKDIR /go/src/seaweedfs/weed

RUN sed -i -e 's/\bDataShardsCount\s*=\s*6\b/DataShardsCount = 10/g' storage/erasure_coding/ec_encoder.go
RUN sed -i -e 's/\bParityShardsCount\s*=\s*6\b/ParityShardsCount = 4/g' storage/erasure_coding/ec_encoder.go

RUN  GO_ENABLED=0 GOOS=linux \
  go build -gcflags=all="-N -l" -tags "large_disk 5BytesOffset" -ldflags "-extldflags -static -X github.com/chrislusf/seaweedfs/weed/util.COMMIT=$(git rev-parse --short HEAD)" -o /go/bin

FROM docker.io/rust:1.62-buster as build-rfs

COPY --from=build-swfs /go/bin/weed /bin/
WORKDIR /src/
COPY . /src/

RUN apt-get update && apt-get -y install libssl-dev cmake gcc fuse3 libfuse-dev pkg-config > /dev/null

RUN cargo build --release && mv ./target/release/rfs /bin/

# RUN SWCOMMIT=$(git rev-parse --short HEAD) \
#   SWLDFLAGS="-X github.com/chrislusf/seaweedfs/weed/util.COMMIT=$SWCOMMIT" \
#   tags="large_disk 5BytesOffset" \
#   GO_ENABLED=0 GOOS=linux \
#   go build -gcflags=all="-N -l" -tags "$tags" -ldflags "-extldflags -static $SWLDFLAGS" -o /go/bin

# FROM gcr.io/distroless/static-debian11
# COPY --from=build /go/bin/weed /
# ENTRYPOINT ["/weed"]

# tar -czvf ./weed.tar.gz weed
# sudo cp ./weed /usr/local/bin/weed

