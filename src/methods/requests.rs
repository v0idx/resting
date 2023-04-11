use resting::Method;
use reqwest::{Client, Response};
use std::future::Future;

use crate::methods::files::fmt_response;

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

pub async fn head(uri: String) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let client = Client::new();
    let request = client.head(uri);
    let response = request.send();

    return response;
}