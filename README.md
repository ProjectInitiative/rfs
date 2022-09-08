# rfs

Passion project trying to rewrite [Seaweedfs](https://github.com/seaweedfs/seaweedfs) in pure rust. The main goal is simplicity, maintainability, and data safety.



## Building from Source

Install dependencies and build with cargo

```bash
sudo apt install libssl-dev cmake gcc fuse3 libfuse-dev pkg-config
cd rfs
cargo build --release
```

Build with docker

```bash
cd rfs
bash ./builders/build-docker.sh
```



