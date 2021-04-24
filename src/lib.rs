//! # webmention
//!
//! Provides support for sending and receiving webmentions. [Webmention](https://www.w3.org/TR/webmention/)
//! is a W3C recommendation for notifying that a link from one URL to another was published.
//!
//! In short: to receive webmentions you need to process POST requests with `source` and `target`
//! on advertised endpoint URL. There are various ways to advertise your endpoint, the simplest of all
//! is to place `<link rel="webmention" href="endpoint_url">` inside of your document.
//!
//! To send webmentions you need to discover endpoint and send a POST request with
//! `source` and `target` payload.
//!
//! ## CLI tool
//!
//! There is a CLI tool available with support for sending webmentions, and (optionally) a simple endpoint
//! based on Rocket.

pub mod error;
pub(crate) mod html;
pub(crate) mod http_client;
pub(crate) mod link_header;
pub mod webmention;
pub(crate) mod wm_url;
pub mod endpoint_discovery;
pub mod storage;

use crate::error::WebmentionError;
use crate::webmention::Webmention;
use crate::wm_url::Url;

use crate::http_client::get;

pub async fn send_mentions_for_link(u: &Url) -> Result<(), WebmentionError> {
    let response = get(u).await?;
    let links = response.html.find_links().await;

    for link in links.into_iter() {
        Webmention::from((u.clone(), link)).send().await?;
    }
    Ok(())
}

pub async fn fetch_links(u: &Url) -> Result<std::collections::HashSet<Url>, WebmentionError> {
    let response = get(u).await?;
    let links = response.html.find_links().await;

    Ok(links.into_iter().collect())
}

use crate::storage::WebmentionStorage;

pub async fn receive_webmention(
    storage: &impl WebmentionStorage,
    source: &Url,
    target: &Url,
) -> Result<bool, WebmentionError> {
    let mut mention = Webmention::from((source.clone(), target.clone()));
    if mention.check().await? {
        println!("Storing webmention {:?}", mention);
        storage
            .store(mention)
            .map_err(|source| WebmentionError::StorageError {
                source: Box::new(source),
            })?;
        return Ok(true);
    } else {
        Ok(false)
    }
}

