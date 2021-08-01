use crate::error::WebmentionError;
use crate::webmention::Webmention;

use crate::wm_url::Url;
use std::sync::{Arc, Mutex};

pub trait WebmentionStorage {
    fn store(&self, webmention: Webmention) -> Result<(), WebmentionError>;
    fn lookup_by_target(&self, target: Url) -> Result<Vec<Webmention>, WebmentionError>;
}

#[derive(Debug)]
pub struct InMemoryWebmentionStorage {
    mentions: Arc<Mutex<Vec<Webmention>>>,
}

impl InMemoryWebmentionStorage {
    pub fn new() -> InMemoryWebmentionStorage {
        InMemoryWebmentionStorage {
            mentions: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WebmentionStorage for InMemoryWebmentionStorage {
    fn store(&self, mention: Webmention) -> Result<(), WebmentionError> {
        {
            let mut lock = self.mentions.lock().unwrap();
            lock.push(mention);
        }
        Ok(())
    }

    fn lookup_by_target(&self, url: Url) -> Result<Vec<Webmention>, WebmentionError> {
        let mut view: Vec<Webmention> = Vec::new();
        let lock = self.mentions.lock().unwrap();
        for mention in lock.iter() {
            if mention.target.eq(&url) {
                view.push(mention.clone());
            }
        }
        Ok(view)
    }
}
