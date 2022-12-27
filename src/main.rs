use serde_json::Value;
use std::env;

struct Item {
    name: String,
    price: f64,
    quantity: u64,
    total: f64,
    url: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let search_term = &args[1];
    let limit = &args[2];
    let url = ("https://au.rs-online.com/web/c/?limit=".to_owned()
        + limit
        + "&searchTerm="
        + search_term)
        .replace(" ", "+");

    let mut items: Vec<Item> = vec![];

    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let cell_selector = scraper::Selector::parse("div > section > script").unwrap();
    let cells = document.select(&cell_selector);

    let script = cells.last().unwrap().html();
    let removed_leading = script.split("productContainer: ").nth(1).unwrap();
    let removed_trailing = removed_leading.split("categories").nth(0).unwrap();

    let no_white_space = removed_trailing.replace(" ", "");
    let no_backslash = no_white_space.replace("\\", "");
    let no_newline = no_backslash.replace("\n", "");
    let no_trailing_comma = &no_newline[0..no_newline.len() - 1];

    let json_data: Value = serde_json::from_str(no_trailing_comma).unwrap();
    // println!("{:#}", json_data);
    if let Some(products) = json_data["products"].as_array() {
        for product in products {
            items.push(Item {
                name: product["manufacturersPartNumber"].to_string(),
                price: product["price"]["unitPrice"].as_f64().unwrap(),
                quantity: product["price"]["packSize"].as_u64().unwrap(),
                total: product["price"]["unitPrice"].as_f64().unwrap()
                    * product["price"]["packSize"].as_f64().unwrap(),
                url: "https://au.rs-online.com".to_string()
                    + &(product["productPath"].to_string())
                        [1..(product["productPath"].to_string()).len() - 1],
            });
        }
    }

    items.sort_by(|a, b| a.total.partial_cmp(&b.total).unwrap());
    items.iter().for_each(|item| {
        println!(
            "{0: <28}: ${1: <8} x {2: <8} {3: <80}",
            item.name, item.price, item.quantity, item.url
        )
    });
}
