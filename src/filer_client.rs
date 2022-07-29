use crate::{errors::{FilerParseError, FilerRequestError}, filer_pb::seaweed_filer_client::SeaweedFilerClient};

use reqwest::Url;
use tokio::runtime::Runtime;
use tonic::{Request, transport::Channel};

pub struct FilerClient {
    pub rt: Runtime,
    pub filer_urls: Vec<Url>,
    pub client: SeaweedFilerClient<Channel>,
}

impl FilerClient {
    pub fn new(filers: String) -> Result<Self, Box<dyn std::error::Error>> {
        
        let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

        let filer_urls: Vec<Url> = match filers.split(",").collect::<Vec<&str>>()
            .iter().map(
                |filer_str|
                {
                    match parse_str_to_url(filer_str.to_string())
                    {
                        Ok(url) => {
                            if !url.has_host()
                            {
                                return Err(FilerParseError::new("missing filer scheme or host address, rerun with -h|--help to see proper format"));
                            }
                            return Ok(url);
                        },
                        Err(e) => { 
                            return Err(FilerParseError::new(format!("parsing {filer_str:?}: {e:?}").as_str()));
                        }
                    };
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
                    let url_str = url.to_string().to_owned();
                    Channel::from_shared(url_str)
                }).filter_map(|endpoint| endpoint.ok());
            
                let channel = Channel::balance_list(endpoints);
                
                SeaweedFilerClient::new(channel)
            });


            return Ok(FilerClient
            {
                rt: rt,
                filer_urls: filer_urls,
                client: client
            });
            
           
        
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