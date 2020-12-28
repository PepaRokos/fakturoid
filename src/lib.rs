//! # fakturoid.cz API library
//!
//! The Rust interface to online accounting service [Fakturoid](http://fakturoid.cz/).
//!
//! ## Features
//!
//! - Account detail
//! - Subjects: create, update, delete, list, filters and fulltext
//! - Invoices: create, update, delete, list, filters and fulltext, invoice actions

pub mod models;
pub mod client;
pub mod error;
pub mod filters;

#[cfg(test)]
mod tests {
    use crate::client::Fakturoid;
    use crate::error::Kind;
    use crate::models::Invoice;

    #[test]
    fn test_connect() {
        let client = Fakturoid::new(
            "fake@user.com",
            "apicode",
            "testslug",
            Some("Rust API client TEST (pepa@bukova.info)")
        );

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let result = client.account().await;
            assert!(result.is_err());
            assert_eq!(*result.err().unwrap().kind(), Kind::Unauthorized);
        });
    }

    #[test]
    fn test_serialize() {
        let mut invoice = Invoice::default();
        invoice.note = Some("Some note".to_string());
        let ser = serde_json::to_string(&invoice);
        assert!(ser.is_ok());
        assert_eq!(ser.unwrap().as_str(), "{\"note\":\"Some note\"}");
    }
}
