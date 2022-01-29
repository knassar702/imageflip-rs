#[path = "http.rs"] mod http;
#[cfg(feature = "webapi")]
#[path = "api.rs"] mod api;
use scraper::{Html, Selector};
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
    let ferg = Html::parse_fragment(&htm);
    let select = Selector::parse("div").unwrap();
    for i in ferg.select(&select) {
        i.value().attr("class").map(|x| {
            if x.contains("base-unit clearfix") {
                let select = Selector::parse("img").unwrap();
                for c in i.select(&select) {
                    let mut url_info = HashMap::new();
                    c.value().attr("src").map(|x| {
                        url_info.insert("url".to_owned(),x.to_owned());
                    });
                    c.value().attr("alt").map(|x| {
                        url_info.insert("title".to_owned(),x.to_owned());
                    });
                    url_info.insert("id".to_owned(),id.to_string());
                    urls.push(url_info.clone());
                }
            }         
        });
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
        let v = start(Some(2));
        assert_eq!(v.len(),5);
    }
    #[cfg(feature = "webapi")]
    #[test]
    fn test_start_api() {
        start_api();
    }
}
