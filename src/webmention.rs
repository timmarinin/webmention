use crate::{
    endpoint_discovery::find_target_endpoint, error::WebmentionError, http_client::get, wm_url::Url,
};
use serde::{Deserialize, Serialize};

/// Contains source URL and target URL, as well as whether we checked the source and whether we
/// sent webmention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webmention {
    pub source: Url,
    pub target: Url,
    checked: Option<bool>,
    sent: bool,
}

/// The logical result of an attempt to send webmention, if there are no [errors](WebmentionError) (such as networking
/// errors, URL parsing errors, etc.)
#[derive(Debug, PartialEq)]
pub enum WebmentionAcceptance {
    /// Common option as of 2021
    NoTargetEndpoint,
    /// Source doesn't contain link to target
    NotValid,
    /// Target endpoint didn't accept the webmention
    NotAccepted,
    /// Target endpoint accepted the webmention
    Accepted,
}

impl Webmention {
    /// Create new Webmention from two `AsRef<str>`, which would be parsed by `Url::parse`.
    pub fn new<T: AsRef<str>>(source: T, target: T) -> Result<Webmention, url::ParseError> {
        let source_url = Url::parse(source.as_ref())?;
        let target_url = Url::parse(target.as_ref())?;
        Ok(Webmention::from((source_url, target_url)))
    }

    /// Send Webmention to target endpoint.
    ///
    /// This includes a) checking the source to include link to target, b) discovering target
    /// endpoint, c) sending POST request to target endpoint.
    ///
    /// You can skip a) via `webmention.set_checked(true)`.
    ///
    /// The result it `WebmentionAcceptance`, which signifies several distinct outcomes.
    pub async fn send(&mut self) -> Result<WebmentionAcceptance, WebmentionError> {
        let valid = if let Some(cached_valid) = self.checked {
            cached_valid
        } else {
            let valid = self.check().await.is_ok();
            self.checked = Some(valid);
            valid
        };

        if !valid {
            return Ok(WebmentionAcceptance::NotValid);
        }

        let endpoint_result = find_target_endpoint(&self.target).await.map_err(|e| {
            WebmentionError::DiscoveryRequestFailed {
                source: Box::new(e),
                url: self.target.clone(),
            }
        })?;

        if endpoint_result.is_none() {
            return Ok(WebmentionAcceptance::NoTargetEndpoint);
        }

        let endpoint = endpoint_result.ok_or(WebmentionError::UnparseableDocument)?;

        let accepted = crate::http_client::post(&endpoint, &self).await?;
        self.sent = true;
        match accepted {
            true => Ok(WebmentionAcceptance::Accepted),
            false => Ok(WebmentionAcceptance::NotAccepted),
        }
    }

    pub async fn check(&mut self) -> Result<(), WebmentionError> {
        let response = get(&self.source).await?;
        response.html.contains(&self.target)
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = Some(checked);
    }
}

impl From<(&Url, &Url)> for Webmention {
    fn from(tuple: (&Url, &Url)) -> Webmention {
        Webmention {
            source: tuple.0.clone(),
            target: tuple.1.clone(),
            sent: false,
            checked: None,
        }
    }
}

impl From<(Url, Url)> for Webmention {
    fn from(tuple: (Url, Url)) -> Webmention {
        Webmention {
            source: tuple.0,
            target: tuple.1,
            sent: false,
            checked: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Webmention, WebmentionAcceptance};
    use crate::wm_url::Url;
    use tokio_test::block_on;

    #[ignore]
    #[test]
    fn webmention_check_test() {
        let source = Url::parse("https://marinintim.com/notes/2021/hwc-rsvp/").unwrap();
        let target = Url::parse("https://evgenykuznetsov.org/events/2021/hwc-online/").unwrap();
        let mut mention = Webmention::from((source, target));
        let result = block_on(mention.check());
        assert!(result.is_ok());
    }

    #[ignore]
    #[test]
    fn webmention_new_test() {
        let wm = Webmention::new(
            "https://marinintim.com/notes/2021/hwc-rsvp/",
            "https://evgenykuznetsov.org/events/2021/hwc-online/",
        );
        assert!(wm.is_ok());
        let mut wm = wm.unwrap();
        wm.set_checked(true); // to skip check
        let result = block_on(wm.send());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, WebmentionAcceptance::Accepted);
    }
}
