use crate::wm_url::Url;
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Class, Name, Not, Predicate},
};

#[derive(Debug)]
pub struct HTML {
    source: Url,
    raw_html: String,
    doc: Option<Document>,
}

impl HTML {
    pub fn new(url: Url, raw_html: String) -> HTML {
        let mut html = HTML {
            source: url,
            raw_html,
            doc: None,
        };
        html.doc = Some(Document::from(html.raw_html.as_str()));
        html
    }

    pub fn contains(&self, target: &Url) -> bool {
        let doc = self.doc.as_ref().unwrap();
        let links = Name("a").and(Attr("href", target.as_str()));
        doc.find(links).count() > 0
    }

    pub fn doc(&self) -> &Document {
        &self.doc.as_ref().unwrap()
    }

    pub async fn find_links(self: &HTML) -> Vec<Url> {
        let mut links = Vec::new();

        let content_link = Name("a").and(Not(Class("u-url")));

        let nodes: Vec<Node>;

        let doc = self.doc.as_ref().unwrap();

        let entries = doc.find(Class("h-entry"));

        if entries.count() > 0 {
            nodes = doc
                .find(Class("h-entry").descendant(content_link))
                .into_iter()
                .collect();
        } else {
            nodes = doc.find(content_link).into_iter().collect();
        }
        for node in nodes.iter() {
            let href = node.attr("href");
            if href.is_some() {
                links.push(href.unwrap().to_string());
            }
        }
        links
            .iter()
            .map(|l| Url::parse(l))
            .filter_map(|u| u.ok())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::http_client::get;
    use crate::wm_url::Url;
    use tokio_test::block_on;
    #[test]
    fn find_links_test() {
        let url = Url::parse("https://marinintim.com/notes/2021/hwc-rsvp/").unwrap();
        let response = block_on(get(&url)).unwrap();
        let links = block_on(response.html.find_links());
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
