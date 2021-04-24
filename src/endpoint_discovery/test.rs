use super::find_target_endpoint;
use crate::wm_url::Url;
use async_std::task::block_on;

#[test]
fn find_target_endpoint_test() {
    let url = Url::parse("https://marinintim.com/notes/2021/hwc-rsvp/").unwrap();
    let result = block_on(find_target_endpoint(&url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(
        result.as_str(),
        "https://webmention.io/marinintim.com/webmention"
    );
}

#[test]
fn webmention_rocks_discovery_endpoint_test1() {
    let url = Url::parse("https://webmention.rocks/test/1").unwrap();
    let result = block_on(find_target_endpoint(&url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(
        result.as_str(),
        "https://webmention.rocks/test/1/webmention"
    );
}

#[test]
fn webmention_rocks_discovery_endpoint_test2() {
    let url = Url::parse("https://webmention.rocks/test/2").unwrap();
    let result = block_on(find_target_endpoint(&url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(
        result.as_str(),
        "https://webmention.rocks/test/2/webmention"
    );
}

#[test]
fn webmention_rocks_discovery_endpoint_test3() {
    let url = Url::parse("https://webmention.rocks/test/3").unwrap();
    let result = block_on(find_target_endpoint(&url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(
        result.as_str(),
        "https://webmention.rocks/test/3/webmention"
    );
}

#[test]
fn webmention_rocks_discovery_endpoint_test4() {
    let url = Url::parse("https://webmention.rocks/test/4").unwrap();
    let result = block_on(find_target_endpoint(&url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(
        result.as_str(),
        "https://webmention.rocks/test/4/webmention"
    );
}

#[test]
fn webmention_rocks_discovery_endpoint_test10() {
    let url = Url::parse("https://webmention.rocks/test/10").unwrap();
    let result = block_on(find_target_endpoint(&url));
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(
        result.as_str(),
        "https://webmention.rocks/test/10/webmention"
    );
}
