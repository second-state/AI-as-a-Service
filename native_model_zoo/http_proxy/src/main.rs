use std::env;
use std::error::Error;
use std::io::{self, Read, Write};
use reqwest::RequestBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let method: &str = &args[1].to_lowercase();
    let url: &str = &args[2];
    let headers: &str = &args[3];

    let client = reqwest::Client::new();
    let mut rb : RequestBuilder = client.get(url);
    if method == "post" {
        rb = client.post(url);
    }

    let hs: serde_json::Value = serde_json::from_str(headers).unwrap();
    for (key, value) in hs.as_object().unwrap() {
        rb = rb.header(key, value.as_str().unwrap());
    }

    let mut buffer: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    rb = rb.body(buffer);

    let resp = rb.send().await?;
    io::stdout().write_all(&resp.bytes().await?)?;

    Ok(())
}
