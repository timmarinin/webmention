use crate::wm_url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webmention {
    pub source: Url,
    pub target: Url,
}

impl From<(&Url, &Url)> for Webmention {
    fn from(tuple: (&Url, &Url)) -> Webmention {
        Webmention {
            source: tuple.0.clone(),
            target: tuple.1.clone(),
        }
    }
}

impl From<(Url, Url)> for Webmention {
    fn from(tuple: (Url, Url)) -> Webmention {
        Webmention {
            source: tuple.0,
            target: tuple.1,
        }
    }
}
