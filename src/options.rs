use clap::Parser;

/// connect FUSE mount to a seaweedfs cluster
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// [SCHEME://HOST:PORT],[filer2],[filer3]
    /// seaweedfs filers (gRPC port) ," delimited,
    /// defaults to first in list, following round robin upon failures
    #[clap(long = "filer", value_parser, default_value_t = String::from("http://localhost:8888"))]
    pub filer_addr: String,

    /// filer remote directory
    #[clap(value_parser)]
    pub filer_remote_dir: String,

    /// system mount directory
    #[clap(value_parser)]
    pub mnt_dir: String,

    /// debug logging takes precedence over -q|--quiet
    #[clap(long, takes_value = false)]
    pub debug: bool,

    /// quiet logging
    #[clap(short, long, takes_value = false)]
    pub quiet: bool,
}

