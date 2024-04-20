use reqwest::blocking::Client;
use scraper::{Html, Selector};
//use serde_json::Value;
use std::env;

// struct Item {
//     name: String,
//     price: f64,
//     quantity: u64,
//     total: f64,
//     url: String,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

//    let search_term = &args[1];
//    let limit = &args[2];
    let search_term = "stm32f103";
    let limit = "20";

    let url = ("https://au.rs-online.com/web/c/?pn=1".to_owned()
        + "&searchTerm="
        + search_term
        + "&rpp="
        + limit)
        .replace(" ", "+");

    let client = Client::new();
    let response = client.get(&url).send().unwrap();
    let resp_url = response.url().to_owned();
    let resp_txt = response.text().unwrap().to_owned();
    let document = Html::parse_document(&resp_txt);

    if url.as_str() != resp_url.as_str() {
        println!("REDIRECT!");
        return;
    }

    //let cell_selector = Selector::parse("section > table > row > cell").unwrap();
    let cell_selector = Selector::parse("section").unwrap();
    let _cell = document.select(&cell_selector);
    dbg!(_cell);

//     let script = cell.html();
//     let removed_leading = script.split("productContainer: ").nth(1).unwrap();
//     let removed_trailing = removed_leading.split("categories").nth(0).unwrap();

//     let no_white_space = removed_trailing.replace(" ", "");
//     let no_backslash = no_white_space.replace("\\", "");
//     let no_newline = no_backslash.replace("\n", "");
//     let no_trailing_comma = &no_newline[0..no_newline.len() - 1];

//     let json_data: Value = serde_json::from_str(no_trailing_comma).unwrap();
//     // println!("{:#}", json_data);
//     let mut items: Vec<Item> = vec![];

//     if let Some(products) = json_data["products"].as_array() {
//         for product in products {
//             items.push(Item {
//                 name: product["manufacturersPartNumber"].to_string(),
//                 price: product["price"]["unitPrice"].as_f64().unwrap(),
//                 quantity: product["price"]["packSize"].as_u64().unwrap(),
//                 total: product["price"]["unitPrice"].as_f64().unwrap()
//                     * product["price"]["packSize"].as_f64().unwrap(),
//                 url: "https://au.rs-online.com".to_string()
//                     + &(product["productPath"].to_string())
//                         [1..(product["productPath"].to_string()).len() - 1],
//             });
//         }
//     }

//     items.sort_by(|a, b| a.total.partial_cmp(&b.total).unwrap());
//     items.iter().for_each(|item| {
//         println!(
//             "{0: <28}: ${1: <8} x {2: <8} {3: <80}",
//             item.name, item.price, item.quantity, item.url
//         )
//     });
}
