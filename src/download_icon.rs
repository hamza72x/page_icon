use std::fs::File;
use std::io::{prelude::*, Error, ErrorKind};
use super::Icon;

fn download_icon(url: &str) -> Result<Icon, Error> {
    let client = reqwest::blocking::Client::new();
    // custom headers
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/48.0.2564.116 Safari/537.36"),
    );

    let response = client
        .get(url)
        .headers(headers)
        .send();

    match response {
        Ok(response) => {
            if response.status().is_success() {
                parse_response(response, url)
            } else {
                Err(Error::new(ErrorKind::Other, "Invalid response"))
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(Error::new(ErrorKind::Other, "Invalid response"))
        }
    }
}

fn parse_response(mut response: reqwest::blocking::Response, url: &str) -> Result<Icon, Error> {

    let mut icon_data = Vec::new();
    response.copy_to(&mut icon_data).unwrap();

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");


    let ext = if content_type.contains("png") {
        ".png"
    } else if content_type.contains("jpeg") {
        ".jpeg"
    } else if content_type.contains("jpg") {
        "jpg"
    } else if content_type.contains("ico") {
        "ico"
    } else {
        ""
    };
    
    if ext == "" {
        return Err(Error::new(ErrorKind::Other, "Invalid content type"));
    }

    let mut file_path = std::env::temp_dir();
    file_path.push(format!("icon{}", ext));
    let mut file = File::create(&file_path)?;
    file.write_all(&icon_data)?;

    let file_size = file.metadata().unwrap().len();

    let icon = Icon {
        source: url.to_owned(),
        data: icon_data,
        ext: ext.to_owned(),
        size: file_size as usize,
        mime: content_type.to_owned(),
    };

    Ok(icon)
}

pub fn download_icons(icon_urls: &[String]) -> Result<Vec<Icon>, reqwest::Error> {
    let mut results = Vec::new();
    for url in icon_urls {
        let icon = download_icon(url);
        if icon.is_ok() {
            println!("url: {}", url);
            results.push(icon.unwrap());
        } else {
            // println!("url: {}, error: {}", url, icon.err().unwrap());
        }
    }
    Ok(results)
}
