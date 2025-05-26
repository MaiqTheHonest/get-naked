use std::{collections::HashSet, process};
use std::fs::{read_to_string, write};


pub fn run() -> std::io::Result<HashSet<String>> {
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
    
    let mut links: HashSet<String> = HashSet::new();

    for _title in body.select(&title_selector) {
        
        if let Some(href) = _title.value().attr("href") {
            if !href.contains("/links-") {
                links.insert(href.to_string());
            }
        }
    }
    
    let links_covered = load_links()?;
    let links_to_post: HashSet<String> = links.difference(&links_covered).cloned().collect();
    save_links(links)?;

    Ok(links_to_post)
}



fn load_links() -> std::io::Result<HashSet<String>> {

    let links = match read_to_string("src/links.txt") {
        Ok(v) => v,
        _ => String::from("none")
    };

    Ok(links.lines().map(|line| line.to_string()).collect())
}



fn save_links(links: HashSet<String>) -> std::io::Result<()> {
    let mut text_links = String::new();

    for link in links.iter() {
        text_links += format!("{}\n", link).as_str();
    }
    write("src/links.txt", &text_links)?;

    Ok(())
}