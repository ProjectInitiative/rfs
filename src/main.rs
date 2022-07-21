
use clap::Parser;
// use swfs::Swfs;
use fuse_mount2::HelloFS;
use fuser::MountOption;
use inode::InodeTable;
use url::Url;
use filer_pb::seaweed_filer_client::SeaweedFilerClient;
use filer_pb::LookupDirectoryEntryRequest;


#[macro_use]
extern crate log;

mod options;
mod filer_requests;
mod swfs;
// mod fuse_mount;
mod fuse_mount2;
mod filer_utils;
mod inode;


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



pub mod filer_pb {
    tonic::include_proto!("filer_pb");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // collect arguments
    let args = options::Args::parse();

    log::set_logger(&LOGGER).unwrap();
    if args.debug { log::set_max_level(log::LevelFilter::Debug); }
    else if args.quiet { log::set_max_level(log::LevelFilter::Off) }
    else { log::set_max_level(log::LevelFilter::Info); }
    

    let filer_urls = &args.filer_addr
    .split("|").collect::<Vec<&str>>()
    .iter().map(
        |filer_str| 
        {
            let filer_url = match parse_str_to_url(filer_str.to_string())
            {
                Ok(url) => {
                    if !url.has_host()
                    {
                        error!("missing filer scheme or host address, rerun with -h|--help to see proper format");
                        std::process::exit(1);
                    }
                    info!("attempting to connect to filer: {:?}",url.as_str());
                    return url;
                },
                Err(e) => { 
                    error!("parsing url: {e:?}");
                    std::process::exit(1);
                }
            };
        }
    ).collect::<Vec<Url>>();
    
    // env_logger::init();
    let mountpoint = args.mnt_dir;
    let mut options = vec![MountOption::RO, MountOption::FSName("hello".to_string())];
    // if matches.is_present("auto_unmount") {
    //     options.push(MountOption::AutoUnmount);
    // }
    // if matches.is_present("allow-root") {
    //     options.push(MountOption::AllowRoot);
    // }

    // let swfs = Swfs { 
    //     base_urls: filer_urls.to_vec(),
    //     inode_table: InodeTable::new()
    // };

    // let filesystem = Swfs {
    //     target: mountpoint.into(),
    // };

    // fuser::mount2(swfs, mountpoint, &options);
    // fuser::mount2(HelloFS, mountpoint, &options);

    let first_filer = filer_urls[0].as_str().to_string();

    let mut client = SeaweedFilerClient::connect(first_filer).await.unwrap();

    let request = tonic::Request::new(LookupDirectoryEntryRequest {
        name: "test1".into(),
        directory: "/".into(),
    });

    let response = client.lookup_directory_entry(request).await.unwrap();

    info!("RESPONSE={:?}", response);

    Ok(())
    
}

fn parse_str_to_url(url_str: String) -> Result<Url, url::ParseError>
{
    return Url::parse(url_str.as_str());
}

