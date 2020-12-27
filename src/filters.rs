use std::collections::HashMap;
use chrono::{DateTime, Local};
use crate::models::InvoiceState;

/// Filter builder trait for implement concrete filtering.
pub trait FilterBuilder {
    /// Builds filter as HashMap
    fn build(&self, filter: Filter) -> HashMap<String, String>;
}

/// Common filter struct.
#[derive(Default, Clone)]
pub struct Filter {
    query_map: HashMap<String, String>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: i32) -> Self {
        self.query_map
            .insert("page".to_string(), format!("{}", page));
        self
    }

    pub fn since(mut self, since: DateTime<Local>) -> Self {
        self.query_map
            .insert("since".to_string(), since.to_rfc3339());
        self
    }

    pub fn updated_since(mut self, upd_since: DateTime<Local>) -> Self {
        self.query_map
            .insert("updated_since".to_string(), upd_since.to_rfc3339());
        self
    }

    pub fn custom_id(mut self, custom_id: &str) -> Self {
        self.query_map
            .insert("custom_id".to_string(), custom_id.to_string());
        self
    }

    pub fn until(mut self, until: DateTime<Local>) -> Self {
        self.query_map
            .insert("until".to_string(), until.to_rfc3339());
        self
    }

    pub fn updated_until(mut self, upd_until: DateTime<Local>) -> Self {
        self.query_map
            .insert("updated_until".to_string(), upd_until.to_rfc3339());
        self
    }

    pub fn number(mut self, number: &str) -> Self {
        self.query_map
            .insert("number".to_string(), number.to_string());
        self
    }

    pub fn status(mut self, status: InvoiceState) -> Self {
        self.query_map
            .insert("status".to_string(), status.to_string());
        self
    }

    pub fn subject_id(mut self, id: i32) -> Self {
        self.query_map
            .insert("subject_id".to_string(), format!("{}", id));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.query_map.is_empty()
    }
}

pub(crate) struct NoneFilter;
pub(crate) struct SubjectFilter;
pub(crate) struct InvoiceFilter;

impl FilterBuilder for NoneFilter {
    fn build(&self, _filter: Filter) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl FilterBuilder for SubjectFilter {
    fn build(&self, filter: Filter) -> HashMap<String, String> {
        filter
            .query_map
            .iter()
            .filter(|&f| {
                *f.0 != "subject_id"
                    && *f.0 != "until"
                    && *f.0 != "updated_until"
                    && *f.0 != "number"
                    && *f.0 != "status"
            })
            .map(|f| (f.0.clone(), f.1.clone()))
            .collect()
    }
}

impl FilterBuilder for InvoiceFilter {
    fn build(&self, filter: Filter) -> HashMap<String, String> {
        filter.query_map
    }
}
