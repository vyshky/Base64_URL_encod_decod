mod libs;
use libs::converter::*;
use libs::bit::*;

fn main() {
    // w1w_readbyte();
    let result = base64encode("privet");
    println!("{}", result);
}
