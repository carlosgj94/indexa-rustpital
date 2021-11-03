use reqwest::Error;
use serde::{Serialize, de::DeserializeOwned};

pub trait HttpServer {
    fn get<T, D>(&self, params: &D) -> Result<T, Error> where T: DeserializeOwned, D: Serialize;
}
pub struct Client {
    pub endpoint: String,
}

impl HttpServer for Client {
    #[tokio::main]
    async fn get<T, D>(&self, req_params: &D) -> Result<T, Error> where T: DeserializeOwned, D: Serialize {
        let resp : T = reqwest::Client::new()
            .get(&self.endpoint)
            .json(&req_params)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }
}
