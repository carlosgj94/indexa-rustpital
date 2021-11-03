use reqwest::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait HttpServer {
    fn get<T, D>(&self, params: &D) -> Result<T, Error>
    where
        T: DeserializeOwned,
        D: Serialize;
}
pub struct Client {
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidResponse<T> {
    jsonrpc: String,
    id: u128,
    result: T,
}

impl HttpServer for Client {
    #[tokio::main]
    async fn get<T, D>(&self, req_params: &D) -> Result<T, Error>
    where
        T: DeserializeOwned,
        D: Serialize,
    {
        let resp: ValidResponse<T> = reqwest::Client::new()
            .get(&self.endpoint)
            .json(&req_params)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.result)
    }
}
