use resting::Method;
use reqwest::{Client, Response};
use std::future::Future;
use std::path::PathBuf;

use crate::methods::files::fmt_response;

pub async fn make_request(method: Method, uri: String, file_path: Option<String>, port: Option<u16>) -> String {
    let mut addr: String;
    let file_path = to_path(file_path);
    if is_ip(&uri) {
        addr = String::from(format!("http://{}", uri));
    } else {
        addr = String::from(uri);
    }
    if port.is_some() {
        addr.push_str(format!(":{}",port.unwrap()).as_str());
    }
    match method {
        Method::HEAD => {
            let req = head(addr).await;
            let ret_string = fmt_response(req, file_path).await;
            return ret_string.unwrap();
        },
        _ => println!(""),
    }
    
    
    return String::new();
}

pub async fn head(uri: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let client = Client::new();
    let request = client.head(uri);
    let response = request.send();

    return response;
}

fn to_path(str: Option<String>) -> Option<PathBuf> {
    if str.is_some() {
        let mut ret_path = PathBuf::new();
        ret_path.push(str.unwrap());
        return Some(ret_path);
    } else {
        return None;
    }
    
}

fn is_ip(addr: &String) -> bool {
    use std::net::IpAddr;
    match IpAddr::parse_ascii(addr.as_bytes()) {
        Ok(_) => return true,
        Err(_) => return false,
    }
    
}