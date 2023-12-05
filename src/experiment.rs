use anyhow::Result;
use arti_client::{config::TorClientConfigBuilder, TorClient};
use arti_hyper::ArtiHttpConnector;
use std::path::Path;
use tls_api::{TlsConnector as TlsConnectorTrait, TlsConnectorBuilder};

#[cfg(not(target_vendor = "apple"))]
use tls_api_native_tls::TlsConnector;
#[cfg(target_vendor = "apple")]
use tls_api_openssl::TlsConnector;

#[tokio::main]
async fn main() -> Result<()> {
    let tor_base_dir = String::from(r#"C:\Users\Ethan\source\tuari-plugin-tor"#);
    let tor_state_dir = tor_base_dir.clone() + r#"\state\"#;
    let tor_cache_dir = tor_base_dir + r#"\cache\"#;
    let config = TorClientConfigBuilder::from_directories(
        Path::new(&tor_state_dir),
        Path::new(&tor_cache_dir),
    ).build()?;

    let tor_client = TorClient::create_bootstrapped(config).await?;

    let tls_connector = TlsConnector::builder()?.build()?;

    let tor_connector = ArtiHttpConnector::new(tor_client, tls_connector);
    let http = hyper::Client::builder().build::<_, hyper::Body>(tor_connector);

    let mut resp = http.get("https://example.com".try_into()?).await?;

    let body = hyper::body::to_bytes(resp.body_mut()).await?;

    println!("Got it from google \n{}", std::str::from_utf8(&body)?);

    Ok(())
}
