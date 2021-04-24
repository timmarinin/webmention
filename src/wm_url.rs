pub use url::Url;

pub fn absolute_url(candidate_url: &str, original_url: &Url) -> Result<Url, url::ParseError> {
    if "".eq(candidate_url) {
        return Ok(original_url.clone());
    }

    match Url::parse(candidate_url) {
        Ok(absolute_url) => Ok(absolute_url),
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let url = original_url.clone();
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

    #[test]
    fn test_absolute_url2() {
        let base_url = Url::parse("https://webmention.rocks/test/22").unwrap();
        let relative_path = "22/webmention";
        assert_eq!(
            absolute_url(relative_path, &base_url).unwrap().as_str(),
            "https://webmention.rocks/test/22/webmention"
        );
    }

    #[test]
    fn test_absolute_url3() {
        let base_url = Url::parse("https://webmention.rocks/test/23/page/wiKQ8pZzlN0q3hsIZADg").unwrap();
        let relative_path = "webmention-endpoint/xXNLydslCJo3niJSfoXk";
        assert_eq!(
            absolute_url(relative_path, &base_url).unwrap().as_str(),
            "https://webmention.rocks/test/23/page/webmention-endpoint/xXNLydslCJo3niJSfoXk"
        );
    }
}
