#[path = "http.rs"] mod http;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;

pub fn start(id: Option<i32>) -> Vec<HashMap<String,String>> {
    let id = match id {
        Some(id) => {id},
        None => {
            1 as i32
        }
    };
    let mut urls:Vec<HashMap<String,String>> = vec![]; 
    for page_id in 0..id {
        let htm = http::http_get(&format!("https://imgflip.com/?page={}",page_id.to_owned())).unwrap();
        let document = Document::from(htm.as_str());
        for node in document.find(Attr("class","base-unit clearfix").descendant(Name("div"))) {
            let mut url_info = HashMap::new();
            let img = match node.find(Class("base-img")).next() {
                Some(img) => {img},
                None => continue,
            };
            match img.attr("src") {
                Some(src) => {url_info.insert("url".to_string(), src.to_string())},
                None => continue,
            };
            match img.attr("alt") {
                Some(alt) => {url_info.insert("alt".to_string(), alt.to_string())},
                None => continue,
            };
            url_info.insert("id".to_string(), format!("{}",page_id));
            urls.push(url_info);
        }
    }
    urls
}
