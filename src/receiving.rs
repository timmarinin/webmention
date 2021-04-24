use crate::storage::WebmentionStorage;
use crate::webmention::Webmention;
use crate::error::WebmentionError;
use crate::checking::check_webmention;
use url::Url;

pub async fn receive_webmention(
    storage: &impl WebmentionStorage,
    source: &Url,
    target: &Url,
) -> Result<bool, WebmentionError> {
    let mention = Webmention::from((source.clone(), target.clone()));
    if check_webmention(&mention).await? {
        println!("Storing webmention {:?}", mention);
        storage
            .store(mention)
            .map_err(|source| WebmentionError::StorageError {
                source: Box::new(source),
            })?;
        return Ok(true);
    } else {
        Ok(false)
    }
}
