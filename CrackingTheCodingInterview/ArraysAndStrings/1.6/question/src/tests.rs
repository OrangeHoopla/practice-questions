

#[cfg(test)]
mod tests {

    use crate::*;
    

    #[test]
    fn true_result() {

        let value = string_compression("wow");
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), "w1o1w1");
    }

    #[test]
    fn book() {

        let value = string_compression("aabcccccaa");
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), "a2b1c5a2");
    }

}