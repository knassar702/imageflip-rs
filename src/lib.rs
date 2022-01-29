#[path = "http.rs"] mod http;
#[cfg(feature = "webapi")]
#[path = "api.rs"] mod api;

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
    let htm = http::http_get(&format!("https://imgflip.com/?page={}",id.to_owned())).unwrap();
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
        url_info.insert("id".to_string(), format!("{}",id));
        urls.push(url_info);
    }
    urls
}

#[cfg(feature = "webapi")]
pub fn start_api(){
    api::main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start() {
        start(Some(1));
    }
    #[cfg(feature = "webapi")]
    #[test]
    fn test_start_api() {
        start_api();
    }
}
