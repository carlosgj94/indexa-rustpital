use serde::{Serialize, Deserialize};
use serde_json::json;
use rpc::HttpServer;
use std::error::Error;


#[derive(Debug, Serialize, Deserialize)]
struct RequestParams {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct EthereumBlock {
    hash: String,
    number: String,
    transactions: serde_json::Value
}

fn main() -> Result<(), Box<dyn Error>> {
    let params = json!(["0x4", true]);
    let req_params = RequestParams {
        jsonrpc: "2.0".to_string(),
        method: "eth_getBlockByNumber".to_string(),
        id: 1,
        params,
    };
    let client = rpc::Client{endpoint:"http://localhost:8546".to_string()};
    let resp : Result<EthereumBlock, _> = client.get(&req_params);

    match resp {
        Ok(r) => {
            println!("{:#?}", r);
            Ok(())
        },
        Err(e) => panic!("Error getting a block {:#?}", e),
    }
}
