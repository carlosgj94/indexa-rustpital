// use ethereum::GethClient;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let geth = ethereum::GethClient::new("http://localhost:8546");
    let block = geth.get_block_by_number("0x44A");
    println!("{:#?}", block);
    let balance = geth.get_balance("0xc17ffed94e0aff6b1d02e6c36bcf3e472b6b0e3d");
    println!("{:#?}", balance);
    Ok(())
}
