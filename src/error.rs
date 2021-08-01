use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum WebmentionError {
    #[error("no endpoint was discovered for {0}")]
    NoEndpointDiscovered(Url),
    #[error("discovery request failed for URL <{url}>")]
    DiscoveryRequestFailed {
        url: Url,
        #[source]
        source: Box<WebmentionError>,
    },
    #[error("webmention request failed for URL <{url}>")]
    SendingRequestFailed {
        url: Url,
        #[source]
        source: anyhow::Error,
    },

    #[error("failed to read data from request for URL <{url}>")]
    RequestFailedRecv {
        url: Url,
        #[source]
        source: anyhow::Error,
    },

    #[error("could not serialize form")]
    UnserializableForm {
        #[source]
        source: anyhow::Error,
    },

    #[error("webmention was not accepted by endpoint <{endpoint}>")]
    NotAccepted {
        endpoint: String,
        status_code: reqwest::StatusCode,
    },

    #[error("generic request failed for URL <{url}>")]
    RequestFailed {
        url: Url,
        #[source]
        source: anyhow::Error,
    },

    #[error("could not store webmention")]
    StorageError {
        #[source]
        source: Box<WebmentionError>,
    },

    #[error("invalid LINK header: {0}")]
    InvalidLinkHeader(String),

    #[error("too many redirects while trying to fetch <{url}>")]
    TooManyRedirects { url: Url },

    #[error("could not parse url")]
    UnparseableUrl {
        #[from]
        source: url::ParseError,
    },
}
