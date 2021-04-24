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

#[cfg(test)]
mod test {
    use crate::webmention::Webmention;
    use crate::wm_url::Url;
    use async_std::task::block_on;
    #[ignore]
    #[test]
    fn send_webmention_test() {
        let source = Url::parse("https://marinintim.com/projects/webmention/").unwrap();
        let target = Url::parse("https://webmention.rocks/test/4").unwrap();
        let mut mention = Webmention::from((source, target));

        let result = block_on(mention.send());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, true);
    }
}
