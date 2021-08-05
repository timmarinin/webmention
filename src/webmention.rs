use crate::{
    endpoint_discovery::find_target_endpoint, error::WebmentionError, http_client::get, wm_url::Url,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webmention {
    pub source: Url,
    pub target: Url,
    checked: Option<bool>,
    sent: bool,
}

pub enum WebmentionAcceptance {
    NotValid,
    NotAccepted,
    Accepted,
}

impl Webmention {
    pub async fn send(&mut self) -> Result<WebmentionAcceptance, WebmentionError> {
        let valid = if let Some(cached_valid) = self.checked {
            cached_valid
        } else {
            let valid = self.check().await?;
            self.checked = Some(valid);
            valid
        };

        if !valid {
            return Ok(WebmentionAcceptance::NotValid);
        }

        let endpoint = find_target_endpoint(&self.target)
            .await
            .map_err(|e| WebmentionError::DiscoveryRequestFailed {
                source: Box::new(e),
                url: self.target.clone(),
            })?
            .ok_or_else(|| WebmentionError::NoEndpointDiscovered(self.target.clone()))?;

        let accepted = crate::http_client::post(&endpoint, &self).await?;
        self.sent = true;
        match accepted {
            true => Ok(WebmentionAcceptance::Accepted),
            false => Ok(WebmentionAcceptance::NotAccepted)
        }
    }

    pub async fn check(&mut self) -> Result<bool, WebmentionError> {
        let response = get(&self.source).await?;
        Ok(response.html.contains(&self.target))
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
    use super::Webmention;
    use crate::wm_url::Url;
    use tokio_test::block_on;
    #[test]
    fn webmention_check_test() {
        let source = Url::parse("https://marinintim.com/notes/2021/hwc-rsvp/").unwrap();
        let target = Url::parse("https://evgenykuznetsov.org/events/2021/hwc-online/").unwrap();
        let mut mention = Webmention::from((source, target));
        let result = block_on(mention.check());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, true);
    }
}
