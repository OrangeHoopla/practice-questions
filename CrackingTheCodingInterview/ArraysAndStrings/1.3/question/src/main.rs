use regex::Regex;


pub mod tests;

fn main() {
    println!("Hello, world!");
}

pub fn URLify(value: String) -> Result<String, String> {

    let mut result = "".to_owned();
    let mut check = true;
    for e in value.trim().chars() {
        if e == ' ' {
            if check {
                result.push('%');
                result.push('2');
                result.push('0');
                check = false;
            }
        }
        else {
            result.push(e);
            check = true;
        }
        
    }
    
    // let re = Regex::new(r" /  +/g").unwrap();
    // let result = value.trim();
    // let result = re.replace_all(result, "%20");
    

    // println!("{}",result.to_owned().len());
    return Ok(result.to_string());
}
