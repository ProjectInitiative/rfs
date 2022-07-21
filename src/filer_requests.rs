use reqwest::{Response, RequestBuilder};
use serde_json::json;
use url::{Url, ParseError};

struct Get {
    params: serde_json::Value
}
impl Default for Get {
    fn default() -> Get {
        Get {
            params: json!({
                "metaData": "true", // get file metadata
                "resolveManifests": "", // resolve manifest chunks
                "limit": "100", // how many file to show
                "lastFileName": "", // the last file in previous batch
                "namePattern": "", // match file names, case-sensitive wildcard characters '*' and '?'
                "namePatternExclude": "" // nagetive match file names, case-sensitive wildcard characters '*' and '?'
            })
        }
    }
}
struct PostPut {
    params: serde_json::Value
}
impl Default for PostPut {
    fn default() -> PostPut {
        PostPut {
            params: json!({
                "dataCenter":"", // data center
                "rack":"", // rack
                "dataNode":"", // data node
                "collection":"", // collection
                "replication":"", // replication
                "fsync":"", // if "true", the file content write will incur an fsync operation (though the file metadata will still be separate)
                "ttl":"", // time to live, examples, 3m: 3 minutes, 4h: 4 hours, 5d: 5 days, 6w: 6 weeks, 7M: 7 months, 8y: 8 years
                "maxMB":"", // max chunk size
                "mode":"", // file mode
                "op":"", // file operation, currently only support "append"
                "skipCheckParentDir":"", // Ensuring parent directory exists cost one metadata API call. Skipping this can reduce network latency.
            })
        }
        // might add another var for headers
    }
}
struct Delete {
    params: serde_json::Value
}
impl Default for Delete {
    fn default() -> Delete {
        Delete {
            params: json!({
                "recurse": "", // if "recursive=true", recursively delete all files and folders
                "ignoreRecurseError":"", // if "ignoreRecursiveError=true", ignore errors in recursive mode
                "skipChunkDeletion":"" // if "skipChunkDeletion=true", do not delete file chunks on volume servers
        })
        }
    }
}

pub trait FilerHTTPRequests {
    fn get(&mut self) -> reqwest::RequestBuilder;
    fn post(&mut self) -> reqwest::RequestBuilder;
    fn put(&mut self) -> reqwest::RequestBuilder;
    fn delete(&mut self) -> reqwest::RequestBuilder;
    fn get_blocking(&mut self) -> reqwest::blocking::RequestBuilder;
    fn post_blocking(&mut self) -> reqwest::blocking::RequestBuilder;
    fn put_blocking(&mut self) -> reqwest::blocking::RequestBuilder;
    fn delete_blocking(&mut self) -> reqwest::blocking::RequestBuilder;
}
pub struct FilerInfo{
    pub url: Url,
}

impl FilerHTTPRequests for FilerInfo {
    fn get(&mut self) -> reqwest::RequestBuilder {
        // parse parameters
        let params = Get::default().params;
        let params_vec = get_params_as_vec(params);
        
        // build HTTP request
        let client = reqwest::Client::new();
        let request = client
            .get(self.url.as_str())
            .query(&params_vec)
            .header("Accept", "application/json");
        return request;
    }

    fn post(&mut self) -> reqwest::RequestBuilder {
        todo!()
    }

    fn put(&mut self) -> reqwest::RequestBuilder {
        todo!()
    }

    fn delete(&mut self) -> reqwest::RequestBuilder {
        todo!()
    }

    fn get_blocking(&mut self) -> reqwest::blocking::RequestBuilder {
        // parse parameters
        let params = Get::default().params;
        let params_vec = get_params_as_vec(params);
        
        // build HTTP request
        let client = reqwest::blocking::Client::new();
        let request = client
            .get(self.url.as_str())
            .query(&params_vec)
            .header("Accept", "application/json");
        return request;
    }

    fn post_blocking(&mut self) -> reqwest::blocking::RequestBuilder {
        todo!()
    }

    fn put_blocking(&mut self) -> reqwest::blocking::RequestBuilder {
        todo!()
    }

    fn delete_blocking(&mut self) -> reqwest::blocking::RequestBuilder {
        todo!()
    }

}


fn get_params_as_vec(params: serde_json::Value) -> Vec<(String, String)>
{
    let mut vec = Vec::new();

    // check if there are valid parameters, and add them to vector
    for (key, value) in params.as_object().unwrap() {
        if key != ""
        {
            vec.push((key.as_str().to_string(), value.as_str().unwrap().to_string()));
        }
    }
    return vec;
}

pub fn remove_last_path_url(url: Url) -> Result<(Url, String), url::ParseError>
{
    let mut path_segments = match url.path_segments().map(|c| c.collect::<Vec<_>>())
    {
        Some(segments) => segments,
        None => Vec::new()
    };
    let mut last_path = match path_segments.pop()
    {
        Some(last_path) => last_path,
        None => ""
    };
    let new_url = match  url.join("./")
    {
        Ok(url) => url,
        Err(e) => return Err(e)
    };
    if last_path == "" { last_path = "/" };
    return Ok((new_url, last_path.to_string()));
}