use crate::error::WebmentionError;
use crate::webmention::Webmention;
use crate::wm_url::Url;

use crate::http_client::get;

pub async fn send_mentions_for_link(u: &Url) -> Result<(), WebmentionError> {
    let response = get(u).await?;
    let links = response.html.find_links().await;

    for link in links.into_iter() {
        send_webmention(Webmention::from((u.clone(), link))).await?;
    }
    Ok(())
}

pub async fn fetch_links(u: &Url) -> Result<std::collections::HashSet<Url>, WebmentionError> {
    let response = get(u).await?;
    let links = response.html.find_links().await;

    Ok(links.into_iter().collect())
}

pub async fn send_webmention(mention: Webmention) -> Result<bool, WebmentionError> {
    let valid = crate::checking::check_webmention(&mention).await?;

    if !valid {
        return Ok(false);
    }

    let endpoint = crate::endpoint_discovery::find_target_endpoint(&mention.target)
        .await
        .map_err(|e| WebmentionError::DiscoveryRequestFailed {
            source: Box::new(e),
            url: mention.target.clone(),
        })?
        .ok_or_else(|| WebmentionError::NoEndpointDiscovered(mention.target.clone()))?;

    let accepted = crate::http_client::post(&endpoint, &mention).await?;

    Ok(accepted)
}

#[cfg(test)]
mod test {
    use super::send_webmention;
    use crate::webmention::Webmention;
    use crate::wm_url::Url;
    use async_std::task::block_on;
    #[ignore]
    #[test]
    fn send_webmention_test() {
        let source = Url::parse("https://marinintim.com/projects/webmention/").unwrap();
        let target = Url::parse("https://webmention.rocks/test/4").unwrap();
        let mention = Webmention::from((source, target));

        let result = block_on(send_webmention(mention));
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, true);
    }
}
