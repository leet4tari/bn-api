//#![deny(unreachable_patterns)]
//#![deny(unused_variables)]
//#![deny(unused_imports)]
//// Unused results is more often than not an error
//#![deny(unused_must_use)]
#[macro_use]
extern crate derive_error;
extern crate reqwest;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate logging;
extern crate log;
extern crate serde;

use log::Level::Debug;
use reqwest::StatusCode;

pub struct BranchClient {
    pub links: LinksResource,
}

impl BranchClient {
    pub fn new(url: String, api_key: String) -> BranchClient {
        BranchClient {
            links: LinksResource::new(&url, api_key),
        }
    }
}

pub struct LinksResource {
    url: String,
    branch_key: String,
}

impl LinksResource {
    fn new(url: &str, branch_key: String) -> LinksResource {
        LinksResource {
            url: format!("{}/url", url),
            branch_key,
        }
    }

    pub fn create(&self, link: DeepLink) -> Result<String, BranchError> {
        let client = reqwest::Client::new();
        let link = BranchApiRequest {
            data: link,
            branch_key: &self.branch_key,
        };
        let mut resp = client.post(&self.url).json(&link).send()?;
        let value: serde_json::Value = resp.json()?;
        jlog!(Debug, "Response from Branch", { "response": &value });

        let status = resp.status();
        if status != StatusCode::OK {
            return Err(resp.error_for_status().err().map(|e| e.into()).unwrap_or(
                BranchError::UnexpectedResponseError(format!(
                    "Unexpected status code from Branch: {}",
                    status
                )),
            ));
        };
        #[derive(Deserialize)]
        struct R {
            url: String,
        }
        let r: R = serde_json::from_value(value)?;

        Ok(r.url)
    }
}

#[derive(Serialize)]
struct BranchApiRequest<'a> {
    branch_key: &'a str,
    #[serde(flatten)]
    data: DeepLink,
}

#[derive(Serialize, Default)]
pub struct DeepLink {
    pub channel: Option<String>,
    pub campaign: Option<String>,
    pub feature: Option<String>,
    pub tags: Vec<String>,
    pub data: DeepLinkData,
}

#[derive(Serialize, Default)]
pub struct DeepLinkData {
    #[serde(rename = "$canonical_identifier")]
    pub cannonical_identifier: Option<String>,
    #[serde(rename = "$og_description")]
    pub description: Option<String>,
    #[serde(rename = "$og_title")]
    pub title: Option<String>,
    #[serde(rename = "$og_image_url")]
    pub image_url: Option<String>,
    #[serde(rename = "$desktop_url")]
    pub desktop_url: Option<String>,
}

#[derive(Debug, Error)]
pub enum BranchError {
    HttpError(reqwest::Error),
    #[error(msg_embedded, no_from, non_std)]
    UnexpectedResponseError(String),
    DeserializationError(serde_json::Error),
}

pub mod prelude {
    pub use super::*;
}
