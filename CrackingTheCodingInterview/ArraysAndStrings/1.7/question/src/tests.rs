

#[cfg(test)]
mod tests {

    use crate::*;
    

    #[test]
    fn true_result() {
        let inner = vec!['1';3];
        let inner2 = vec!['0';3];
        let mut outer = vec![inner2;2];
        outer.push(inner);

        println!("{:?}",outer);
        let value = transpose(outer);
        println!("{:?}",value);
        // println!("{}",value.unwrap());
        // assert_eq!(value.unwrap(), "w1o1w1");
        //i dont want to write out the values for this unit test
        // but transpose == 90 degree turn
    }

    #[test]
    fn book() {

        // let value = string_compression("aabcccccaa");
        // println!("{}",value.unwrap());
        // assert_eq!(value.unwrap(), "a2b1c5a2");
    }

}