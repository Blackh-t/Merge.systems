#[derive(Debug, serde::Serialize)]
struct Person {
    name: String,
    phone: String,
    address: String,
}

pub use reqwest::get;

async fn search_name(quary: &str) -> Vec<Person> {
    let mut person_lists = Vec::new();
    // Fetch the target HTML document
    let response = get(&format!("https://www.180.no/search/persons?w={}", quary)).await;
    // Get the HTML content from the request response
    let html_content = response.unwrap().text().await;
    // Parse content into HTML
    let document = scraper::Html::parse_document(&html_content.unwrap());
    // Define Selector
    let html_target_selector = scraper::Selector::parse("div.container-fluid").unwrap();
    // Apply the CSS selector to get all person info.
    let html_list = document.select(&html_target_selector);

    // Iter & Store contents
    for person in html_list {
        let name = person
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .map(|a| a.text().collect::<Vec<_>>().join(""));
        let address = person
            .select(&scraper::Selector::parse("span.cc-info").unwrap())
            .next()
            .map(|a| a.text().collect::<Vec<_>>().join(""));
        let phone = person
            .select(&scraper::Selector::parse("a[data-phone]").unwrap())
            .next()
            .and_then(|a| a.value().attr("data-phone"))
            .map(str::to_owned);

        person_lists.push(Person {
            name: name.unwrap_or_default(),
            phone: phone.unwrap_or_default(),
            address: address.unwrap_or_default(),
        });
    }
    person_lists
}

// Referace: https://www.zenrows.com/blog/rust-web-scraping#extract-html-data
//#[cfg(test)]
//mod test {
//    use super::*;
//    use serde_json;
//
//    #[tokio::test]
//    async fn test_scaper() {
//        let res = search_name("Narongchai").await;
//        let pretty_res = serde_json::to_string_pretty(&res).unwrap();
//        assert!(pretty_res.contains("Narongchai"))
//    }
//}
