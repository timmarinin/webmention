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
/// Defines document
pub mod html;
/// Defines http_client that is used for GETting and POSTing
pub mod http_client;
/// Defines utility to deal with LINK header
pub mod link_header;

/// Specifies the endpoint discovery algorithm
pub mod endpoint_discovery;
/// Defines interface for webmention storage
#[cfg(feature = "receive")]
pub mod storage;
pub mod webmention;
/// Defines utility to deal with URLs.
pub(crate) mod wm_url;

/// Various error conditions that could happen during processing webmentions
pub use crate::error::WebmentionError;
/// Source URL and target URL combined with some metadata
pub use crate::webmention::Webmention;

#[cfg(feature = "receive")]
pub async fn receive_webmention(
    storage: &impl crate::storage::WebmentionStorage,
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
