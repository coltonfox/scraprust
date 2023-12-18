use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn main() {
    // URL of the website to scrape
    let url = "https://example.com";

    // Send an HTTP GET request and retrieve the response
    let response = Client::new()
        .get(url)
        .send()
        .expect("Failed to send request");

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().expect("Failed to read response body");

        // Parse the HTML content using the scraper library
        let document = Html::parse_document(&body);

        // Example: Extracting all the links (a tags) from the HTML
        let link_selector = Selector::parse("a").expect("Failed to create selector");

        for link in document.select(&link_selector) {
            let href = link.value().attr("href").unwrap_or("");
            println!("Link: {}", href);
        }
    } else {
        println!("Request failed with status code: {}", response.status());
    }
}
