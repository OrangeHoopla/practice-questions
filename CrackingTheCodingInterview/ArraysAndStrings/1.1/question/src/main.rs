
pub mod tests;

fn main() {
    println!("Hello, world!");
    println!("{}", is_unique("value".to_string()).unwrap());
}

//Assumming ASCII
pub fn is_unique(value: String) -> Result<bool, String> {
    if value.len() > 128 {
        return Ok(false);
    }

    let mut character_map: [u8;128] = [0x00;128];
    for character in value.chars() {
        if character_map[character as usize] == 0 {
            character_map[character as usize] = 1;
        }

        else {
            return Ok(false);
        }
    }

    return Ok(true);
    

}
