use reqwest::blocking::Client;

pub fn http_get(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client.get(url).send()?;
    Ok(res.text().unwrap())
}
