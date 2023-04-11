use std::future::Future;
use reqwest::Response;
use std::error::Error;

use std::fs::File;
use::std::path::PathBuf;
use std::io::prelude::*;

pub async fn fmt_response(res: impl Future<Output = Result<Response, reqwest::Error>>, file_path: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
    let response = res.await?;

    let mut ret_string = String::new();

    let app_string = format!("Status Code: {} {}", response.status().as_str(), response.status().canonical_reason().unwrap());

    ret_string.push_str(app_string.as_str());

    let app_string = format!("\nHeaders: \n {:#?}", response.headers());
    ret_string.push_str(app_string.as_str());

    let body = response.text().await?;

    if !(body.is_empty()) {
        ret_string.push_str(format!("\nBody: {}", body).as_str());
    }

    if file_path.is_some() {
        return write_out(ret_string, file_path.unwrap());
    } else {
        return Ok(ret_string);
    }
}

pub fn write_out(output: String, path: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = File::create(&path)?;
    file.write_all(output.as_bytes())?;
    file.flush()?;

    Ok(String::from(format!("Response written to: {}", path.to_string_lossy())))
}