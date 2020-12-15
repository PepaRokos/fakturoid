use reqwest::Error;
use std::error::Error as StdError;
use std::fmt;
use serde::export::Formatter;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug)]
pub enum Kind {
    ServiceError,
    TooManyRequests,
    PaymentRequired,
    UnprocessableEntity,
    Forbidden,
    Other
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
    errors: HashMap<String, Vec<String>>
}

#[derive(Debug)]
pub struct ApiRequestError {
    kind: Kind,
    inner_request: Option<Error>,
    inner_other: Option<Box<dyn StdError>>,
    data_errors: Option<HashMap<String, Vec<String>>>
}

impl ApiRequestError {
    pub fn into_request_err(self) -> Option<Error> {
        self.inner_request
    }

    pub fn into_std_err(self) -> Box<dyn StdError> {
        assert!(self.inner_request.is_some() || self.inner_other.is_some(), "There is no inner error!");
        if let Some(req_err) = self.inner_request {
            req_err.into()
        } else {
            self.inner_other.unwrap()
        }
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn into_data_errors(self) -> Option<HashMap<String, Vec<String>>> {
        self.data_errors
    }

    pub fn data_errors(&self) -> Option<&HashMap<String, Vec<String>>> {
        self.data_errors.as_ref()
    }

    pub(crate) fn from_std_err<E>(err: E) -> Self
    where
        E: StdError + 'static
    {
        Self {
            kind: Kind::Other,
            inner_request: None,
            inner_other: Some(err.into()),
            data_errors: None
        }
    }

    pub(crate) fn from_data(data: DataErrors, err: Error) -> Self {
        Self{
            kind: Kind::UnprocessableEntity,
            inner_request: Some(err),
            inner_other: None,
            data_errors: Some(data.errors)
        }
    }
}

impl From<Error> for ApiRequestError {
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
        }
        Self {
            kind,
            inner_request: Some(err),
            inner_other: None,
            data_errors: None
        }
    }
}

impl fmt::Display for ApiRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::ServiceError => {
                if let Some(req_err) = self.inner_request.as_ref() {
                    f.write_fmt(format_args!("Service Unavailable. Status is: {}", req_err.status().as_ref().unwrap()))
                } else {
                    f.write_str("Service error")
                }
            }
            Kind::TooManyRequests => { f.write_str("Request limit exceeded. Limit is 200 per one minute.") }
            Kind::PaymentRequired => { f.write_str("Payment required") }
            Kind::UnprocessableEntity => {
                if let Some(errs) = self.data_errors.as_ref() {
                    f.write_fmt(format_args!("Errors in input data: {:?}", errs))
                }else {
                    f.write_str("Malformed input data.")
                }
            }
            Kind::Forbidden => { f.write_str("Forbidden operation") }
            Kind::Other => {
                assert!(self.inner_request.is_some() || self.inner_other.is_some(), "There is no inner error!");
                if let Some(req_err) = self.inner_request.as_ref() {
                    req_err.fmt(f)
                } else {
                    self.inner_other.as_ref().unwrap().fmt(f)
                }
            }
        }
    }
}

impl StdError for ApiRequestError {}