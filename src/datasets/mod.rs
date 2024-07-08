mod open_ml;
mod web_access;
mod error;
mod minst;

#[cfg(test)]
mod tests {
    use super::*;

    use open_ml::FetchOpenMLBuilder;

    #[test]
    fn can_create_builder() {
        let builder = FetchOpenMLBuilder::new();
        println!("builder is {:?}", builder);
    }
}
