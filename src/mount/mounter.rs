use std::{
    ffi::OsStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::mount::{filer_client::FilerClient, swfs::Swfs};

pub fn init_mount(
    filer_addr: String,
    mountpoint: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let filer_client: FilerClient = match FilerClient::new(filer_addr) {
        Ok(filer_client) => filer_client,
        Err(e) => {
            error!("{:?}", e);
            std::process::exit(1);
        }
    };

    // env_logger::init();

    // let rt = Runtime::new().unwrap();
    // let handle = rt.handle().clone();

    let filesystem = Swfs {
        target: mountpoint.clone().into(),
        // rt: tokio::runtime::Builder::new_multi_thread()
        //         .enable_all()
        //         .build()
        //         .unwrap(),
        // handle: Handle::current(),
        filer_client,
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
