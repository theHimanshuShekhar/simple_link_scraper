#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("simple_web_scraper!");

    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("a").unwrap(); // a is the tag for links
    for element in document.select(&selector) {
        let href: &str = element.value().attr("href").unwrap_or_default();
        if href.starts_with("http") {
            println!("link = {href}");
        }
    }

    Ok(())
}
