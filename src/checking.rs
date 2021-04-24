use crate::error::WebmentionError;
use crate::http_client::get;
use crate::webmention::Webmention;

pub async fn check_webmention(mention: &Webmention) -> Result<bool, WebmentionError> {
    let response = get(&mention.source).await?;
    Ok(response.html.contains(&mention.target))
}

#[cfg(test)]
mod test {
    use super::check_webmention;
    use crate::webmention::Webmention;
    use crate::wm_url::Url;
    use async_std::task::block_on;
    #[test]
    fn check_webmention_test() {
        let source = Url::parse("https://marinintim.com/notes/2021/hwc-rsvp/").unwrap();
        let target = Url::parse("https://evgenykuznetsov.org/events/2021/hwc-online/").unwrap();
        let mention = Webmention::from((source, target));
        let result = block_on(check_webmention(&mention));
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, true);
    }
}
