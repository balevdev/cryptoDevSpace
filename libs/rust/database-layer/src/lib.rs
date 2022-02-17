pub fn databaselayer() -> String {
    "databaselayer".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(databaselayer(), "databaselayer".to_string());
    }
}
