mod find_best_icon;
mod get_icon_links;
mod get_page;
mod download_icon;

use clap::Parser;
use get_icon_links::get_icon_links;
use get_page::get_page;
use std::{io::{Write, Error}};

#[derive(Debug)]
pub struct Icon {
    source: String,
    data: Vec<u8>,
    size: usize,
    ext: String,
    mime: String,
}

// only 2 args
// 1. url
// 2. output file path (optional)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "The url to get the icon from")]
    url: String,

    #[arg(short, long, help = "The path to write the icon to")]
    output: Option<String>,
}


fn main() {
    let args = Cli::parse();

    if let Err(e) = download_best_icon(&args.url, args.output.as_deref()) {
        println!("Error: {}", e);
    }
}

pub fn download_best_icon(url: &str, output: Option<&str>) -> Result<(), Error> {

    let page = get_page(&url);
    if page.is_err() {
        return Err(Error::new(std::io::ErrorKind::Other, "Failed to get page"));
    }

    let icon_urls = get_icon_links(&url, &page.unwrap());
    if icon_urls.len() == 0 {
        return Err(Error::new(std::io::ErrorKind::Other, "No icon urls found"));
    }

    let icons = download_icon::download_icons(&icon_urls);
    if icons.is_err() {
        return Err(Error::new(std::io::ErrorKind::Other, format!("error in download_icons: {}", icons.err().unwrap())));
    }
    let mut icons = icons.unwrap();

    if icons.len() == 0 {
        return Err(Error::new(std::io::ErrorKind::Other, format!("no icons found, but icon urls were: {:?}", icon_urls)));
    }

    let best_icon = find_best_icon::find_best_icon(&mut icons, None);
    if best_icon.is_none() {
        return Err(Error::new(std::io::ErrorKind::Other, format!("no best icon found, but icons were: {:?}", icons)));
    }

    let best_icon = best_icon.unwrap();

    // write Icon.data to file
    let mut file_path = match output {
        Some(path) => std::path::PathBuf::from(path),
        None => std::env::temp_dir(),
    };

    file_path.push(format!("icon{}", best_icon.ext));
    let mut file = std::fs::File::create(&file_path).unwrap();
    file.write_all(&best_icon.data).unwrap();

    println!("Wrote icon to file: {:?}", file_path);

    Ok(())
}