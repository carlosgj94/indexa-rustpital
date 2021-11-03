use rpc::HttpServer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

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

pub struct GethClient {}

impl GethClient {
    pub fn get_block_by_number(number: String) -> EthereumBlock {
        let params = json!([number, true]);
        let req_params = RequestParams {
            jsonrpc: "2.0".to_string(),
            method: "eth_getBlockByNumber".to_string(),
            id: 1,
            params,
        };
        let client = rpc::Client {
            endpoint: "http://localhost:8546".to_string(),
        };
        let resp: Result<EthereumBlock, _> = client.get(&req_params);
        match resp {
            Ok(r) => r,
            Err(e) => panic!("Error getting a block {:#?}", e),
        }
    }
}
