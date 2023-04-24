use url::Url;
use select::document::Document;
use select::predicate::{Name};

fn href_is_icon(href: &str) -> bool {
    let regex = regex::Regex::new(r"((icon.*\.(png|jpg))|(\w+\.ico))").unwrap();
    regex.is_match(href)
}

fn get_domain_url(some_url: &str) -> String {
    let parsed_url = Url::parse(some_url).unwrap();
    let mut new_url = parsed_url.clone();
    new_url.set_path("");
    return new_url.into();
}

fn link_tag_links(html: &str) -> Vec<String> {
    let mut links = vec![];
    let document = Document::from(html);
    for link in document.find(Name("link")).filter_map(|n| n.attr("href")) {
        if !href_is_icon(link) {
            continue;
        }
        links.push(link.to_string());
    }
    links
}

fn rel_x_links(html: &str, rel_value: &str) -> Vec<String> {
    let mut links = vec![];
    let document = Document::from(html);
    for link in document.find(Name("link")).filter(|n| n.attr("rel") == Some(rel_value)) {
        if let Some(href) = link.attr("href") {
            links.push(href.to_string());
        }
    }
    links
}

fn meta_tag_links(html: &str) -> Vec<String> {
    let mut links = vec![];
    let document = Document::from(html);
    for meta in document.find(Name("meta")).filter(|n| n.attr("property") == Some("og:image")) {
        if let Some(content) = meta.attr("content") {
            links.push(content.to_string());
        }
    }
    links
}

pub fn get_icon_links(root_url: &str, dom: &str) -> Vec<String> {
    let mut icon_links = vec![];

    icon_links.extend(link_tag_links(dom));
    icon_links.extend(meta_tag_links(dom));
    icon_links.extend(rel_x_links(dom, "icon"));
    icon_links.extend(rel_x_links(dom, "apple-touch-icon"));
    icon_links.extend(rel_x_links(dom, "shortcut icon"));

    let domain_url = get_domain_url(root_url);
    icon_links = icon_links.into_iter()
        .map(|icon_link| Url::parse(&root_url).unwrap().join(&icon_link).unwrap().into())
        .collect();
    icon_links.push(format!("{}apple-touch-icon.png", domain_url));

    icon_links
}
