
//TODO place rayon image code here
#[cfg(test)]
mod tests {
    use crate::sqrt;

    #[test]
    fn this_test_will_pass() {
        let value = sqrt(9.0);
        assert_eq!(value.unwrap(), 3.0);
    }

    #[test]
    fn this_test_will_fail() {
        let value = sqrt(9.0);
        assert_eq!(value.unwrap(), 5.0);
    }
}