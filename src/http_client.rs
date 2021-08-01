use crate::error::WebmentionError;
use crate::html::HTML;
use crate::wm_url::Url;
use serde::Serialize;

pub struct Response {
    pub url: Url,
    pub html: HTML,
    pub rels: std::collections::HashMap<String, Vec<String>>,
}

pub async fn get(u: &Url) -> Result<Response, WebmentionError> {
    let client = reqwest::Client::new();
    let req = client.get(u.clone());

    let res = req
        .send()
        .await
        .map_err(|err| WebmentionError::RequestFailed {
            url: u.clone(),
            source: err.into(),
        })?;

    let link_headers = res.headers().get_all("link");
    let rels = crate::link_header::all_rels(link_headers);

    let response =
        res.text_with_charset("utf-8")
            .await
            .map_err(|err| WebmentionError::RequestFailedRecv {
                url: u.clone(),
                source: err.into(),
            })?;

    return Ok(Response {
        url: u.clone(),
        html: HTML::new(u.clone(), response),
        rels,
    });
}

pub async fn post(endpoint: &Url, body: &impl Serialize) -> Result<bool, WebmentionError> {
    let client = reqwest::Client::new();

    let response = client
        .post(endpoint.clone())
        .form(body)
        .send()
        .await
        .map_err(|source| WebmentionError::SendingRequestFailed {
            url: endpoint.clone(),
            source: source.into(),
        })?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(true),
        reqwest::StatusCode::CREATED => Ok(true),
        reqwest::StatusCode::ACCEPTED => Ok(true),
        status => Err(WebmentionError::NotAccepted {
            endpoint: endpoint.as_str().to_string(),
            status_code: status,
        }),
    }
}

#[cfg(test)]
mod test {
    use super::get;
    use crate::wm_url::Url;
    use tokio_test::block_on;
    #[test]
    fn fetch_url_test() {
        let url: Url = Url::parse("https://httpbin.org/get").unwrap();
        let response = block_on(get(&url));
        assert!(response.is_ok());
    }
}
