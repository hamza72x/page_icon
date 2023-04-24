use reqwest::blocking::get;

pub fn get_page(page_url: &str) -> Result<String, reqwest::Error> {
    let response = get(page_url)?.text()?;
    Ok(response)
}
