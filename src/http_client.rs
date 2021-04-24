use crate::error::WebmentionError;
use crate::html::HTML;
use crate::wm_url::Url;
use serde::Serialize;

type Client = surf::Client;

/// @TODO: handle redirects more gracefully
pub fn http_client() -> Client {
    surf::client()
}

pub struct Response {
    pub url: Url,
    pub html: HTML,
    pub resp: surf::Response,
}

const REDIRECT_STATUS_CODES: &[http_types::StatusCode] = &[
    http_types::StatusCode::MovedPermanently,
    http_types::StatusCode::Found,
    http_types::StatusCode::SeeOther,
    http_types::StatusCode::TemporaryRedirect,
    http_types::StatusCode::PermanentRedirect,
];

pub async fn get(u: &Url) -> Result<Response, WebmentionError> {
    let mut url = u.clone();
    let mut redirects_followed: u8 = 0;

    loop {
        let req = surf::get(&url);
        let client = http_client();

        let mut res = client
            .send(req)
            .await
            .map_err(|err| WebmentionError::RequestFailed {
                url: url.clone(),
                source: err.into_inner(),
            })?;

        if REDIRECT_STATUS_CODES.contains(&res.status()) {
            if let Some(loc) = res.header(http_types::headers::LOCATION) {
                let new_loc = match Url::parse(loc.last().as_str()) {
                    Ok(new_loc) => Ok(new_loc),
                    Err(url::ParseError::RelativeUrlWithoutBase) => {
                        Ok(url.join(loc.last().as_str()).unwrap())
                    }
                    Err(e) => Err(WebmentionError::UnparseableUrl { source: e }),
                }?;
                if redirects_followed < 20 {
                    url = new_loc;
                    redirects_followed += 1;
                    continue;
                } else {
                    return Err(WebmentionError::TooManyRedirects { url: u.clone() });
                }
            }
        }

        let response =
            res.body_string()
                .await
                .map_err(|err| WebmentionError::RequestFailedRecv {
                    url: url.clone(),
                    source: err.into_inner(),
                })?;

        return Ok(Response {
            url: url.clone(),
            html: HTML::new(url.clone(), response),
            resp: res,
        });
    }
}

pub async fn post(endpoint: &Url, body: &impl Serialize) -> Result<bool, WebmentionError> {
    let client = crate::http_client::http_client();

    let body =
        surf::Body::from_form(body).map_err(|source| WebmentionError::UnserializableForm {
            source: source.into_inner(),
        })?;

    let request = surf::post(&endpoint).body(body);

    let response =
        client
            .send(request)
            .await
            .map_err(|source| WebmentionError::SendingRequestFailed {
                url: endpoint.clone(),
                source: source.into_inner(),
            })?;

    match response.status() {
        http_types::StatusCode::Ok => Ok(true),
        http_types::StatusCode::Created => Ok(true),
        http_types::StatusCode::Accepted => Ok(true),
        status => {
            Err(WebmentionError::NotAccepted {
                endpoint: endpoint.as_str().to_string(),
                status_code: status,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::get;
    use crate::wm_url::Url;
    use async_std::task::block_on;
    #[test]
    fn fetch_url_test() {
        let url: Url = Url::parse("https://httpbin.org/get").unwrap();
        let response = block_on(get(&url));
        assert!(response.is_ok());
    }
}
