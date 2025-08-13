use regex::Regex;


pub mod tests;

fn main() {
    println!("Hello, world!");
}

pub fn pal_per(val1: String, val2: String) -> Result<bool, String> {

    let one = val1.clone();
    let two = val2.clone();
    if is_permutation(val1, val2) {

        if is_palindrome(&one) || is_palindrome(&two) {
            return Ok(true)
        }
    }
    return Ok(false);

}

pub fn is_permutation(original: String, check: String ) -> bool {
    
    if original.len() != check.len() {
        return false;
    }
    let mut chars: Vec<char> = original.chars().collect();
    chars.sort();

    let mut chars2: Vec<char> = check.chars().collect();
    chars2.sort();

    let result = chars.iter()
                            .zip(chars2)
                            .all(|(a,b)| *a==b);
    return result;

}


pub fn is_palindrome(value: &str) -> bool {

    return value == value.chars().rev().collect::<String>();
}