

#[cfg(test)]
mod tests {

    use crate::pal_per;

    #[test]
    fn this_test_will_pass() {

        let value = pal_per("taco cat".to_owned(),
        "taco cat".to_owned());
        // println!("{}",value.unwrap());
        assert_eq!(value.unwrap(), true);
    }

}