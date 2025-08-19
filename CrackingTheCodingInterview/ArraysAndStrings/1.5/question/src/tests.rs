

#[cfg(test)]
mod tests {

    use crate::one_away;

    #[test]
    fn true_result() {

        let value = one_away(String::from("pale"),String::from("pale"));
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), true);
    }

    #[test]
    fn false_result() {

        let value = one_away(String::from("pale"),String::from("bate"));
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), false);
    }

}