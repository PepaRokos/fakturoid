use reqwest::Error;
use serde::export::Formatter;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Kind {
    ServiceError,
    TooManyRequests,
    PaymentRequired,
    UnprocessableEntity,
    Forbidden,
    EntityDoesNotExists,
    Other,
}

#[derive(Debug)]
pub struct UnknownError(String);

impl UnknownError {
    pub(crate) fn new(fn_name: &str) -> Self {
        Self(fn_name.to_string())
    }
}

impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Something is wrong in function {}", self.0))
    }
}

impl StdError for UnknownError {}

#[derive(Deserialize)]
pub(crate) struct DataErrors {
    errors: HashMap<String, Vec<String>>,
}

/// If something goes wrong this error wil bew returned.
#[derive(Debug)]
pub struct FakturoidError {
    kind: Kind,
    inner_request: Option<Error>,
    inner_other: Option<Box<dyn StdError>>,
    data_errors: Option<HashMap<String, Vec<String>>>,
}

impl FakturoidError {
    /// Transforms this object into underlying error from reqwest library if there is any.
    pub fn into_request_err(self) -> Option<Error> {
        self.inner_request
    }

    /// Transforms this object into std::error::Error.
    pub fn into_std_err(self) -> Box<dyn StdError> {
        assert!(
            self.inner_request.is_some() || self.inner_other.is_some(),
            "There is no inner error!"
        );
        if let Some(req_err) = self.inner_request {
            req_err.into()
        } else {
            self.inner_other.unwrap()
        }
    }

    /// Error kind.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// If fakturoid.cz API returns JSON with errors (status 422) method transforms this object
    /// into `HashMap` of these errors otherwise `None` will be returned.
    pub fn into_data_errors(self) -> Option<HashMap<String, Vec<String>>> {
        self.data_errors
    }

    /// If fakturoid.cz API returns JSON with errors (status 422) method returns reference to
    /// `HashMap` of these errors otherwise `None` will be returned.
    pub fn data_errors(&self) -> Option<&HashMap<String, Vec<String>>> {
        self.data_errors.as_ref()
    }

    pub(crate) fn from_std_err<E>(err: E) -> Self
    where
        E: StdError + 'static,
    {
        Self {
            kind: Kind::Other,
            inner_request: None,
            inner_other: Some(err.into()),
            data_errors: None,
        }
    }

    pub(crate) fn from_data(data: DataErrors, err: Error) -> Self {
        Self {
            kind: Kind::UnprocessableEntity,
            inner_request: Some(err),
            inner_other: None,
            data_errors: Some(data.errors),
        }
    }
}

impl From<Error> for FakturoidError {
    fn from(err: Error) -> Self {
        let mut kind = Kind::Other;
        if let Some(status) = err.status() {
            if status.is_server_error() {
                kind = Kind::ServiceError;
            }
            if status.as_u16() == 429 {
                kind = Kind::TooManyRequests;
            }
            if status.as_u16() == 402 {
                kind = Kind::PaymentRequired;
            }
            if status.as_u16() == 422 {
                kind = Kind::UnprocessableEntity;
            }
            if status.as_u16() == 403 {
                kind = Kind::Forbidden;
            }
            if status.as_u16() == 404 {
                kind = Kind::EntityDoesNotExists;
            }
        }
        Self {
            kind,
            inner_request: Some(err),
            inner_other: None,
            data_errors: None,
        }
    }
}

impl fmt::Display for FakturoidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::ServiceError => {
                if let Some(req_err) = self.inner_request.as_ref() {
                    f.write_fmt(format_args!(
                        "Service Unavailable. Status is: {}",
                        req_err.status().as_ref().unwrap()
                    ))
                } else {
                    f.write_str("Service error")
                }
            }
            Kind::TooManyRequests => {
                f.write_str("Request limit exceeded. Limit is 200 per one minute.")
            }
            Kind::PaymentRequired => f.write_str("Payment required"),
            Kind::UnprocessableEntity => {
                if let Some(errs) = self.data_errors.as_ref() {
                    f.write_fmt(format_args!("Errors in input data: {:?}", errs))
                } else {
                    f.write_str("Malformed input data.")
                }
            },
            Kind::Forbidden => f.write_str("Forbidden operation"),
            Kind::EntityDoesNotExists => f.write_str("Entity does not exists"),
            Kind::Other => {
                assert!(
                    self.inner_request.is_some() || self.inner_other.is_some(),
                    "There is no inner error!"
                );
                if let Some(req_err) = self.inner_request.as_ref() {
                    req_err.fmt(f)
                } else {
                    self.inner_other.as_ref().unwrap().fmt(f)
                }
            }
        }
    }
}

impl StdError for FakturoidError {}
