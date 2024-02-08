//use futures_util::stream::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize)]
pub struct XtqlQuery {
    pub query: Value,
    pub options: Value,
}

#[derive(Debug)]
pub enum CustomError {
    SerdeJsonError(serde_json::Error),
    ReqwestError(reqwest::Error),
    XtdbError(String),
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::SerdeJsonError(e) => e.fmt(f),
            CustomError::ReqwestError(e) => e.fmt(f),
            CustomError::XtdbError(e) => e.fmt(f),
        }
    }
}

pub struct XtdbClient {
    base_url: String,
    headers: HeaderMap,
    client: Client,
    latest_transaction: Arc<RwLock<Option<u64>>>,
}

impl XtdbClient {
    pub fn new(base_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/jsonl"));
        XtdbClient {
            base_url: base_url.to_string(),
            client: Client::new(),
            headers,
            latest_transaction: Arc::new(RwLock::new(None)),
        }
    }
    pub async fn execute_query(&self, query: XtqlQuery) -> Result<Vec<Value>, CustomError> {
        let url = format!("{}/query", self.base_url);
        let query = json!({
            "query": query.query,
            "queryOpts": query.options,
        });

        let response = self
            .client
            .post(&url)
            .headers(self.headers.clone())
            .json(&query)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let mut values = Vec::new();
                    let body = resp.text().await.map_err(CustomError::ReqwestError)?;
                    for line in body.lines() {
                        let parsed_response: Value =
                            serde_json::from_str(line).map_err(CustomError::SerdeJsonError)?;
                        values.push(parsed_response);
                    }
                    Ok(values)
                } else {
                    // Get the error body text if available
                    let error_body = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| String::from("Failed to retrieve error message"));
                    Err(CustomError::XtdbError(error_body)) // directly return the error body as a string
                }
            }

            Err(err) => Err(CustomError::ReqwestError(err)),
        }
    }

    pub async fn set_latest_transaction(&self, transaction_id: u64) {
        let mut latest_transaction = self.latest_transaction.write().await;
        *latest_transaction = Some(transaction_id);
    }

    pub async fn get_latest_transaction(&self) -> Option<u64> {
        let latest_transaction = self.latest_transaction.read().await;
        *latest_transaction
    }
}
