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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
