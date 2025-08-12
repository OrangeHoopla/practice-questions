

#[cfg(test)]
mod tests {

    use crate::URLify;

    #[test]
    fn this_test_will_pass() {

        let value = URLify("John  cash smith".to_owned());
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), "John%20cash%20smith");
    }

}