mod libs;
use libs::base64::*;

fn main() {
    let result = base64encode("привет");
    println!("{}", result);
}
