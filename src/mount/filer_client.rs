use crate::{errors::{FilerParseError, FilerRequestError}, pb::filer_pb::seaweed_filer_client::SeaweedFilerClient};

use std::sync::{Arc,Mutex};
use reqwest::Url;
use tokio::runtime::Runtime;
use tonic::{Request, transport::Channel, codegen::http::uri::InvalidUri};

pub struct FilerClient {
    pub rt: Runtime,
    pub filer_urls: Vec<Url>,
    filer_url_index: Arc<Mutex<usize>>,
    pub filer_grpc_client: SeaweedFilerClient<Channel>,
    pub filer_http_client: reqwest::Client, 
    // pub client: Mutex<SeaweedFilerClient<Channel>>,
}

impl FilerClient {
    pub fn new(filers: String) -> Result<Self, Box<dyn std::error::Error>> {
        
        let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

        let filer_urls: Vec<Url> = match filers.split(',').collect::<Vec<&str>>()
            .iter().map(
                |filer_str|
                {
                    Ok(match parse_str_to_url(filer_str.to_string())
                    {
                        Ok(url) => {
                            if !url.has_host()
                            {
                                return Err(FilerParseError::new("missing filer scheme or host address, rerun with -h|--help to see proper format"));
                            }
                            url
                            // return Ok(url);
                        },
                        Err(e) => { 
                            return Err(FilerParseError::new(format!("parsing {filer_str:?}: {e:?}").as_str()));
                        }
                    })
                }
            ).collect()
            {
                Ok(filer_urls) => filer_urls,
                Err(e) => {
                    return Err(e)?;
                }
            };

            let client = rt.block_on(async {
                let endpoints = filer_urls
                .iter().map(|url| {
                    let mut url_copy = url.clone();
                    let port = match url_copy.port() {
                        Some(port) => port + 10000,
                        None => 8888 + 10000,
                    };
                    url_copy.set_port(Some(port));
                    let url_str = url_copy.to_string();
                    Channel::from_shared(url_str)
                }).filter_map(|endpoint| endpoint.ok());
            
                let channel = Channel::balance_list(endpoints);
                
                SeaweedFilerClient::new(channel)
            });

            Ok(FilerClient
            {
                rt,
                filer_urls,
                filer_url_index: Arc::new(Mutex::new(0)),
                filer_grpc_client: client,
                filer_http_client: reqwest::Client::new(),
                // client: Mutex::new(client),
            })
            
           
        
    }


    /// thread safe round robin selection for quering a filer from provided list
    pub fn get_next_http_filer(&self) -> Option<reqwest::Url>
    {
        let mut index = match self.filer_url_index.lock()
        {
            Ok(index) => index,
            Err(_) => return None
        };
        let url = self.filer_urls[*index].clone(); 
        if *index != self.filer_urls.len() - 1 { *index += 1; }
        else { *index = 0; }
        drop(index);
        Some(url)
    }
    // pub fn try_filer_request<T>(message: T) -> Result<Request<T>, FilerRequestError>
    // {
    //     let request = tonic::Request::new(message);
    // }

}

fn parse_str_to_url(url_str: String) -> Result<Url, url::ParseError>
{
    return Url::parse(url_str.as_str());
}


// info!("attempting to connect to filers: {:?}",url.as_str());
