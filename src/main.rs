use clap::Parser;
use fuser::MountOption;
use std::ffi::OsStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use swfs::Swfs;

use crate::filer_client::FilerClient;

#[macro_use]
extern crate log;

mod errors;
mod mount;
mod options;
use mount::filer_client;
use mount::filer_requests;
use mount::filer_utils;
use mount::mounter;
use mount::swfs;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        println!("{}: {}: {}", record.target(), record.level(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: ConsoleLogger = ConsoleLogger;

pub mod pb {
    pub mod filer_pb {
        tonic::include_proto!("filer_pb");
    }
    pub mod master_pb {
        tonic::include_proto!("master_pb");
    }
    pub mod volume_pb {
        tonic::include_proto!("volume_server_pb");
    }
    pub mod remote_pb {
        tonic::include_proto!("remote_pb");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::set_logger(&LOGGER).unwrap();

    // collect arguments
    let cli = options::Cli::parse();

    match &cli.command {
        options::Commands::Mount(mount) => {
            mounter::init_mount(mount.filer_addr.clone(), mount.mnt_dir.clone());
        }
    }

    if cli.debug {
        log::set_max_level(log::LevelFilter::Debug);
    } else if cli.quiet {
        log::set_max_level(log::LevelFilter::Off)
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    Ok(())
}
