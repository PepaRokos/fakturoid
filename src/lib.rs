pub mod models;
pub mod client;
pub mod error;
pub mod filters;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
