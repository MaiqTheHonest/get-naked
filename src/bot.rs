use std::{process, env};
use std::error::Error;
use std::collections::HashSet;
use dotenv::dotenv;
use urlencoding::encode;
use std::fs::write;


pub fn run(links: HashSet<String>) -> Result<(), Box<dyn Error>>{

    dotenv().ok();

    let bot_token = env::var("BOT_TOKEN").unwrap_or_else(|_| {
        eprintln!("Missing a bot token (API key) in .env");
        write("links.txt", "");  //clear links
        process::exit(1);
    });

    // to get chat id, copy post id from private channel, paste in browser, prepend -100 to it to get -100**********
    let chat_id = env::var("CHAT_ID").unwrap_or_else(|_| {
        eprintln!("Missing a destination chat id in .env");
        process::exit(1);
    });


    let mut article_count: i8 = 0;
    for mut link in links.into_iter() {
        // encoding into HTML because other wise Telegram will strip off the Instant View rhash
        // also extracting and formatting page title, adding t.me prefix VVV
        link = build_html_link(link); 
        
        let _response = reqwest::blocking::get(format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}&parse_mode=HTML", bot_token, chat_id, link))?;
        article_count += 1;
    }

    println!("Success: Posted {} articles", article_count);
    Ok(())
}



pub fn build_html_link(mut link: String) -> String {

    let link_text: String = link.rsplit("/").next().unwrap()
    .split(".").next()
    .unwrap_or("untitled")
    .split("-").map(|word| capitalize(word))
    .collect::<Vec<_>>().join(" ");
    
    fn capitalize(word: &str) -> String {
        let mut chars = word.chars();
        chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
    }

    link = encode(format!("https://t.me/iv?url={}&rhash=b61af1fd206bb4", link).as_str()).into_owned();    // shadowing

    format!(r#"<a href="{}">{}</a>"#, link, link_text)
}
