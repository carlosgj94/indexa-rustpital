// use ethereum::GethClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let resp = ethereum::GethClient::get_block_by_number("0x44A".to_string());
    println!("{:#?}", resp);
    Ok(())
}
