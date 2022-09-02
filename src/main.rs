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
mod filer_client;
mod filer_requests;
mod filer_utils;
mod fuse_mount2;
mod options;
mod swfs;

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
    // collect arguments
    let args = options::Args::parse();

    log::set_logger(&LOGGER).unwrap();
    if args.debug {
        log::set_max_level(log::LevelFilter::Debug);
    } else if args.quiet {
        log::set_max_level(log::LevelFilter::Off)
    } else {
        log::set_max_level(log::LevelFilter::Info);
    }

    let filer_client: FilerClient = match FilerClient::new(args.filer_addr) {
        Ok(filer_client) => filer_client,
        Err(e) => {
            error!("{:?}", e);
            std::process::exit(1);
        }
    };

    // env_logger::init();
    let mountpoint = args.mnt_dir;

    // let rt = Runtime::new().unwrap();
    // let handle = rt.handle().clone();

    let filesystem = Swfs {
        target: mountpoint.clone().into(),
        // rt: tokio::runtime::Builder::new_multi_thread()
        //         .enable_all()
        //         .build()
        //         .unwrap(),
        // handle: Handle::current(),
        filer_client: filer_client,
    };

    // let request = tonic::Request::new(LookupDirectoryEntryRequest {
    //     name: "test1".into(),
    //     directory: "/".into(),
    // });

    // let response = filer_client.client.lookup_directory_entry(request).await.unwrap();

    // info!("RESPONSE={:?}", response);

    let fuse_args = [
        // OsStr::new("-o"), OsStr::new("allow_other"),
        // OsStr::new("-o"), OsStr::new("auto_unmount"),
        OsStr::new("-o"),
        OsStr::new("max_read=0"),
        OsStr::new("-o"),
        OsStr::new("fsname=swfs"),
    ];

    // fuse_mt::mount(
    //     fuse_mt::FuseMT::new(filesystem, 4),
    //     mountpoint,
    //     &fuse_args[..],
    // )
    // .unwrap();

    let fuse_handle = match fuse_mt::spawn_mount(
        fuse_mt::FuseMT::new(filesystem, 4),
        mountpoint,
        &fuse_args[..],
    ) {
        Ok(fuse_handle) => fuse_handle,
        Err(e) => {
            error!("could not spawn mount {:?}", e);
            return Err(e)?;
        }
    };

    // setup signal termination handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    info!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    info!("Unmounting and Exiting...");

    // unmount and clean up dangling thread
    drop(fuse_handle);

    Ok(())
}
