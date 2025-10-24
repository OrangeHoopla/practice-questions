pub mod tests;
use std::collections::LinkedList;
fn main() {
    println!("Hello, world!");
    println!("{}", is_unique("value".to_string()).unwrap());
}

//Assumming ASCII
pub fn is_unique(value: String) -> Result<bool, String> {

    let list = LinkedList::from([1, 2, 3]);

    return Ok(true);
}
