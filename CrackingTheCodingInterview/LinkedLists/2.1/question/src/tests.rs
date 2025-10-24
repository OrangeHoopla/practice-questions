#[cfg(test)]
mod tests {
    use crate::is_unique;

    #[test]
    fn true_result() {
        let value = is_unique("word".to_owned());
        assert_eq!(value.unwrap(), true);
    }


    
    #[test]
    fn false_result() {
        let value = is_unique("TATTLE".to_owned());
        assert_ne!(value.unwrap(), true);
    }
}
