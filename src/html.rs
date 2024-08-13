use crate::{wm_url::Url, WebmentionError};
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Class, Name, Not, Predicate},
};

#[derive(Debug)]
pub struct HTML {
    _source: Url,
    raw_html: String,
    doc: Option<Document>,
}

impl HTML {
    pub fn new(url: Url, raw_html: String) -> HTML {
        let mut html = HTML {
            _source: url,
            raw_html,
            doc: None,
        };
        html.doc = Some(Document::from(html.raw_html.as_str()));
        html
    }

    pub fn contains(&self, target: &Url) -> Result<(), WebmentionError> {
        let doc = self
            .doc
            .as_ref()
            .ok_or(WebmentionError::UnparseableDocument)?;
        let links = Name("a").and(Attr("href", target.as_str()));

        if doc.find(links).count() > 0 {
            Ok(())
        } else {
            Err(WebmentionError::NoDocumentLinks)
        }
    }

    pub fn doc(&self) -> Result<&Document, WebmentionError> {
        self.doc
            .as_ref()
            .ok_or(WebmentionError::UnparseableDocument)
    }

    pub async fn find_links(self: &HTML) -> Result<Vec<Url>, WebmentionError> {
        let mut links = Vec::new();

        let content_link = Name("a").and(Not(Class("u-url")));

        let doc = self
            .doc
            .as_ref()
            .ok_or(WebmentionError::UnparseableDocument)?;

        let entries = doc.find(Class("h-entry"));

        let nodes: Vec<Node> = if entries.count() > 0 {
            doc.find(Class("h-entry").descendant(content_link))
                .collect()
        } else {
            doc.find(content_link).collect()
        };

        for node in nodes.iter() {
            if let Some(href) = node.attr("href") {
                links.push(href.to_string());
            }
        }
        Ok(links
            .iter()
            .map(|l| Url::parse(l))
            .filter_map(|u| u.ok())
            .collect())
    }
}

#[cfg(test)]
mod test {
    use crate::http_client::get;
    use crate::wm_url::Url;
    use tokio_test::block_on;

    #[ignore]
    #[test]
    fn find_links_test() {
        let url = Url::parse("https://marinintim.com/notes/2021/hwc-rsvp/").unwrap();
        let response = block_on(get(&url)).unwrap();
        let links = block_on(response.html.find_links()).unwrap();
        let links_str: Vec<&str> = links.iter().map(|s| s.as_str()).collect();
        assert_eq!(
            links_str,
            vec![
                "https://evgenykuznetsov.org/events/2021/hwc-online/",
                "https://events.indieweb.org/2021/03/-hwc-09ReXTMBeU3M"
            ]
        );
    }
}
