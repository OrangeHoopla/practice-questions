

#[cfg(test)]
mod tests {

    use crate::*;
    

    #[test]
    fn true_result() {

        let value = is_substring("wow","wow");
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), true);
    }

    #[test]
    fn book() {

        let value = is_substring("waterbottle","erbottlewat");
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), true);
    }

}