


pub mod tests;

fn main() {
    println!("Hello, world!");
}

//this seem like a way of cheating to answer this question
pub fn is_substring(val1: &str,val2: &str) -> Result<bool,String> {

    let tmp = val2.to_string() + val2;

    if tmp.contains(val1) {
        Ok(true)
    }
    else {
        Ok(false)
    }
    
}
