

#[cfg(test)]
mod tests {
    use crate::is_permutation;

    #[test]
    fn this_test_will_pass() {
        let value = is_permutation("real".to_owned(),"real".to_owned());
        assert_eq!(value.unwrap(), true);
    }

    #[test]
    fn testa() {
        let value = is_permutation("leas".to_owned(),"real".to_owned());
        assert_eq!(value.unwrap(), false);
    }

    #[test]
    fn testc() {
        let value = is_permutation("".to_owned(),"real".to_owned());
        assert_eq!(value.unwrap(), false);
    }
}