mod summoner;
pub use self::summoner::*;

use hyper::{Uri};
use hyper::rt::Future;
use failure::Error;
use crate::regions::{WithHosts, Platforms};

pub struct Api {
    api_key: String,
    platform: Platforms,
}

impl Api {
    pub fn new(api_key: String, platform: Platforms) -> Self {
        Self {
            api_key,
            platform,
        }
    }

    pub fn get_url(&self, path: String) -> Uri {
        format!("{}{}", self.platform.host(), path).parse::<Uri>().unwrap()
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type FutureResult<T> = dyn Future<Item=T, Error=Error>;
