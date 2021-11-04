use rpc::HttpServer;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct RequestParams {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumBlock {
    hash: String,
    number: String,
    transactions: serde_json::Value,
}

pub struct GethClient {
    pub client: rpc::Client,
}

impl GethClient {
    pub fn new(endpoint: &str) -> Self {
        GethClient {
            client: rpc::Client {
                endpoint: endpoint.to_string(),
            },
        }
    }

    pub fn get_block_by_number(&self, number: &str) -> EthereumBlock {
        let params = json!([number.to_string(), true]);
        let req_params = GethClient::make_params("eth_getBlockByNumber", params);
        let resp: Result<EthereumBlock, _> = self.client.get(&req_params);
        match resp {
            Ok(r) => r,
            Err(e) => panic!("Error getting a block {:#?}", e),
        }
    }

    fn make_params(method: &str, params: serde_json::Value) -> RequestParams {
        RequestParams {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            id: 1,
            params,
        }
    }
}
