use regex::Regex;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

pub async fn extract_url_content(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 7.0; SM-G930V Build/NRD90M) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/59.0.3071.125 Mobile Safari/537.36")
        .build()
        .unwrap();

    let response = client.get(url).send().await?;
    let html = response.text().await?;
    let parsed_html = Html::parse_document(&html);

    let mut all_elements = parsed_html
        .select(&Selector::parse("main").unwrap())
        .collect::<Vec<_>>();

    if all_elements.is_empty() {
        all_elements = parsed_html
            .select(&Selector::parse("article").unwrap())
            .collect::<Vec<_>>();
    }

    if all_elements.is_empty() {
        all_elements = parsed_html
            .select(&Selector::parse("div").unwrap())
            .collect::<Vec<_>>();
    }

    let mut max_text_len = 0;
    let mut main_content: Option<ElementRef> = None;

    for elem in &all_elements {
        let children = elem.text().collect::<Vec<_>>();
        if !children.is_empty() {
            let total_text_len: usize = children.iter().map(|child| child.len()).sum();

            if total_text_len > max_text_len {
                max_text_len = total_text_len;
                main_content = Some(*elem);
            }
        }
    }

    let excluded = vec![
        "nav",
        "footer",
        "header",
        "script",
        "style",
        "sidebar",
        "content_below",
    ];

    let mut result = Vec::new();

    let element = main_content.unwrap();
    let mut stack = vec![element];

    while let Some(current) = stack.pop() {
        for child in current.children() {
            match child.value() {
                scraper::Node::Element(el) => {
                    if el.name() == "script" {
                        continue;
                    }
                    if el.name() == "style" {
                        continue;
                    }
                    let class = el.attr("class").unwrap_or("");
                    let id = el.attr("id").unwrap_or("");

                    if excluded
                        .iter()
                        .all(|&ex| !class.contains(ex) && !id.contains(ex))
                    {
                        if let Some(el_ref) = ElementRef::wrap(child) {
                            stack.push(el_ref);
                        }
                    }
                }
                scraper::Node::Text(ref text_node) => {
                    let img_regex = Regex::new(r"(?i)<img[^>]*>").unwrap();
                    let trimmed = img_regex.replace_all(text_node.trim(), "");

                    if !trimmed.is_empty() {
                        result.insert(0, trimmed.to_string());
                    }
                }
                _ => (),
            }
        }
    }
    let result = result.join(" ");
    if result.eq(" ") {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}
