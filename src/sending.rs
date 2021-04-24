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
