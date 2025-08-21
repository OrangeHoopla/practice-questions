


pub mod tests;

fn main() {
    println!("Hello, world!");
}

pub fn string_compression(val1: &str) -> Result<String,&str> {

    let mut result = "".to_owned();
    let chars: Vec<char> = val1.trim().chars().collect();
    // chars.sort_by(|b, a| b.cmp(a));

    let mut prev_char = chars[0];
    let last = *chars.last().unwrap();
    let mut count = 0;
    for i in chars {

        if i == prev_char {
            count += 1;
        }
        else if  i != prev_char {
            result.push(prev_char);
            result = result + &count.to_string();
            count = 1;
        }
        prev_char = i;
        }

        result.push(last);
        result = result + &count.to_string();
    
    println!("{}", result);

    Ok(result)
    
}
