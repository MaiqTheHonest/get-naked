use std::{collections::HashSet, process};
use scraper::{Html, Selector};

pub fn run(){
    println!("website ran");

    let response = reqwest::blocking::get("https://www.nakedcapitalism.com");
    let body = match response {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Couldn't access website, check if down. Error code: {:?}", e);
            process::exit(1)
        }
    };

    let body = scraper::Html::parse_document(body.text().unwrap().as_str());

    let title_selector = scraper::Selector::parse("h2.post-title a").unwrap();
    
    let mut links: HashSet<&str> = HashSet::new();

    for _title in body.select(&title_selector) {
        
        if let Some(href) = _title.value().attr("href") {
            if !href.contains("/links-") {
                links.insert(href);
            }
        }
    }
    println!("{:#?}", links)
}

