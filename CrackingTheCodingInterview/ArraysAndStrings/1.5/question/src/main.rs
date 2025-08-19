use regex::Regex;


pub mod tests;

fn main() {
    println!("Hello, world!");
}

pub fn one_away(val1: String,val2: String) -> Result<bool,bool> {

    if val1.len().abs_diff(val2.len()) > 1 {
        return Ok(false);
    }

    let length = std::cmp::min(val1.len(), val2.len());

    let aaa: Vec<char> = val1.chars().collect();
    let bbb: Vec<char> = val2.chars().collect();

    let mut index1: usize = 0;
    let mut index2: usize = 0;
    let mut foundChange = false;
    while val1.len() > index1 && val2.len() > index2 {
        if aaa[index1] != bbb[index2] {
            if foundChange {
                return Ok(false);
            }
            foundChange = true;
            if val1.len() == val2.len() {
                index1 += 1;
            }
        }
        else {
            index1 += 1;
        }
        index2 +=1; //longer string
    }

    return Ok(true);
}
