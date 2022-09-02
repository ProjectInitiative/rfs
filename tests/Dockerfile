FROM docker.io/golang:1.18 as build 


RUN git clone --depth 1 -b master https://github.com/chrislusf/seaweedfs /go/src/seaweedfs
WORKDIR /go/src/seaweedfs/weed

RUN sed -i -e 's/\bDataShardsCount\s*=\s*6\b/DataShardsCount = 10/g' storage/erasure_coding/ec_encoder.go
RUN sed -i -e 's/\bParityShardsCount\s*=\s*6\b/ParityShardsCount = 4/g' storage/erasure_coding/ec_encoder.go

RUN SWCOMMIT=$(git rev-parse --short HEAD) \
  SWLDFLAGS="-X github.com/chrislusf/seaweedfs/weed/util.COMMIT=$SWCOMMIT" \
  tags="large_disk 5BytesOffset" \
  GO_ENABLED=0 GOOS=linux \
  go build -gcflags=all="-N -l" -tags "$tags" -ldflags "-extldflags -static $SWLDFLAGS" -o /go/bin

FROM gcr.io/distroless/static-debian11
COPY --from=build /go/bin/weed /
ENTRYPOINT ["/weed"]

# tar -czvf ./weed.tar.gz weed
# sudo cp ./weed /usr/local/bin/weed

