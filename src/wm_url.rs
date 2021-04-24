pub use url::Url;

pub fn absolute_url(candidate_url: &str, original_url: &Url) -> Result<Url, url::ParseError> {
    match Url::parse(candidate_url) {
        Ok(absolute_url) => Ok(absolute_url),
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let mut url = original_url.clone();
            let segments = url.path_segments().unwrap();
            let last = segments.last().unwrap();
            if !last.contains(".") {
                url.path_segments_mut().unwrap().push("");
            }
            url.join(candidate_url)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::absolute_url;
    use url::Url;
    #[test]
    fn test_absolute_url() {
        let index_html_url = Url::parse("https://marinintim.com/projects/index.html").unwrap();
        let pretty_url = Url::parse("https://marinintim.com/projects").unwrap();
        let relative_path = "api/webmention";

        assert_eq!(
            absolute_url(relative_path, &index_html_url).unwrap().as_str(),
            "https://marinintim.com/projects/api/webmention"
        );
        assert_eq!(
            absolute_url(relative_path, &pretty_url).unwrap().as_str(),
            "https://marinintim.com/projects/api/webmention"
        );
    }
}
