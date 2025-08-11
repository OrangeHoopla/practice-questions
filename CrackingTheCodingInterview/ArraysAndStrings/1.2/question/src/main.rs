
pub mod tests;

fn main() {
    println!("Hello, world!");
}

pub fn is_permutation(original: String, check: String ) -> Result<bool, String> {
    
    if original.len() != check.len() {
        return Ok(false);
    }
    let mut chars: Vec<char> = original.chars().collect();
    chars.sort();

    let mut chars2: Vec<char> = check.chars().collect();
    chars2.sort();

    let result = chars.iter()
                            .zip(chars2)
                            .all(|(a,b)| *a==b);
    return Ok(result);

}
