use regex::Regex;
use reqwest;
use select::document::Document;
use std::error::Error;
use structopt::StructOpt;
use url::Url;

mod regex_patterns;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(try_from_str = Url::parse))]
    url: Url,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let base_url = args.url.as_str();

    let response = reqwest::get(base_url).await?;

    if !response.status().is_success() {
        println!("Failed to retrieve the page: {:?}", response.status());
        return Ok(());
    }

    let body = response.text().await?;
    let document = Document::from_read(body.as_bytes())?;

    // Create a Url instance from the base URL for resolving relative URLs
    let base_url = Url::parse(base_url)?;

    let mut secrets_found = false;
    for node in document.find(select::predicate::Name("script")) {
        if let Some(src) = node.attr("src") {
            // Parse the relative URL and resolve it against the base URL
            let js_url = base_url.join(src)?;
            let js_name = js_url.path_segments().and_then(|s| s.last()).unwrap_or(src);

            let js_response = reqwest::get(js_url.as_str()).await?;
            let js_content = js_response.text().await?;

            for pattern_str in &regex_patterns::patterns::PATTERNS {
                let re = Regex::new(pattern_str)?;
                for secret in re.captures_iter(&js_content) {
                    if let (Some(secret_name), Some(secret_value)) = (secret.get(1), secret.get(2))
                    {
                        println!(
                            "Secret Found in {}: {} = {}",
                            js_name,
                            secret_name.as_str(),
                            secret_value.as_str()
                        );
                        secrets_found = true;
                    }
                }
            }
        }
    }

    if !secrets_found {
        println!("No secrets found.");
    }

    Ok(())
}
