use reqwest::{Client, Response};
use std::error::Error;
use std::future::Future;
use clap::ValueEnum;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    /// GET Request
    GET,
    /// HEAD Request
    HEAD,
    /// POST Request
    POST,
    /// PUT Request
    PUT,
    /// DELETE Request
    DELETE,
    /// CONNECT Request
    CONNECT,
    /// OPTIONS Request
    OPTIONS,
    /// TRACE Request
    TRACE,
    /// PATCH Request
    PATCH,
}

async fn head(uri: String) ->  impl Future<Output = Result<Response, reqwest::Error>>{
    let client = Client::new();
    let request = client.head(uri);
    let response = request.send();

    return response;
}

async fn fmt_response(res: impl Future<Output = Result<Response, reqwest::Error>>) -> Result<String, Box<dyn Error>>{
    let response = res.await?;

    let mut ret_string = String::new();

    let app_string = format!("Status Code: {} {}", response.status().as_str(), response.status().canonical_reason().unwrap());

    ret_string.push_str(app_string.as_str());

    let app_string = format!("\nHeaders: \n {:#?}", response.headers());
    ret_string.push_str(app_string.as_str());

    let body = response.text().await?;

    if body.is_empty() {
        return Ok(ret_string);
    } else {
        ret_string.push_str(format!("\nBody: {}", body).as_str());
        return Ok(ret_string);
    }
}

pub async fn make_request(method: Method, uri: String) -> String {    
    match method {
        Method::HEAD => {
            let req = head(uri).await;
            let ret_string = fmt_response(req).await;
            return ret_string.unwrap();
        },
        _ => println!(""),
    }
    
    
    return String::new();
}

pub fn write_out(output: String, path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::create(path)?;
    file.write_all(output.as_bytes())?;
    file.flush()?;

    Ok(String::from(format!("Response written to: {}", path.to_string_lossy())))
}