use crate::error::WebmentionError;
use crate::http_client::get;
use crate::wm_url::absolute_url;
use crate::wm_url::Url;
use select::node::Node;
use select::predicate::{Name, Predicate};

struct RelWebmention;
impl Predicate for RelWebmention {
    fn matches(&self, node: &Node) -> bool {
        node.attr("rel").map_or(false, |rels| {
            rels.split_whitespace().any(|rel| rel == "webmention")
        })
    }
}

pub async fn find_target_endpoint(url: &Url) -> Result<Option<Url>, WebmentionError> {
    let mut endpoint_candidates: Vec<(usize, Url)> = Vec::new();

    let response = get(url)
        .await
        .map_err(|source| WebmentionError::DiscoveryRequestFailed {
            url: url.clone(),
            source: Box::new(source),
        })?;

    let url = response.url.clone();

    let endpoint_in_link_header = response
        .rels
        .get("webmention")
        .and_then(|urls| urls.first());

    if let Some(link_str) = endpoint_in_link_header {
        if let Ok(u) = absolute_url(link_str, &url) {
            endpoint_candidates.push((0, u));
        }
    }

    let doc = response.html.doc()?;

    {
        let mut link_rels = doc.find(Name("link").and(RelWebmention));

        if let Some(node) = link_rels.next() {
            if let Some(href) = node.attr("href") {
                if let Ok(url) = absolute_url(href, &url) {
                    endpoint_candidates.push((node.index(), url));
                }
            }
        }
    }

    {
        let mut a_rels = doc.find(Name("a").and(RelWebmention));

        if let Some(node) = a_rels.next() {
            if let Some(href) = node.attr("href") {
                if let Ok(url) = absolute_url(href, &url) {
                    endpoint_candidates.push((node.index(), url));
                }
            }
        }
    }
    endpoint_candidates.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(endpoint_candidates.first().map(|s| s.1.clone()))
}

#[cfg(test)]
mod test;
