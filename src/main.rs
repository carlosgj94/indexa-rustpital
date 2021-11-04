// use ethereum::GethClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let geth = ethereum::GethClient::new("http://localhost:8546");
    let block = geth.get_block_by_number("0x44A");
    println!("{:#?}", block);
    Ok(())
}
