pub mod open_ml;
mod web_access;
mod minst;
mod error;

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
