use http::{header, HeaderValue};
use rquest::tls::{chrome, ImpersonateSettings};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a pre-configured TLS settings
    let settings = ImpersonateSettings::builder()
        .tls(chrome::chrome_tls_template_1()?)
        .http2(chrome::chrome_http2_template_1())
        .headers(Box::new(|headers| {
            headers.insert(header::USER_AGENT, HeaderValue::from_static("rquest"));
        }))
        .build();

    // Build a client with pre-configured TLS settings
    let client = rquest::Client::builder()
        .use_preconfigured_tls(settings)
        .build()?;

    // Use the API you're already familiar with
    let resp = client.get("https://tls.peet.ws/api/all").send().await?;
    println!("{}", resp.text().await?);

    Ok(())
}