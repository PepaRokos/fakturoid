use crate::error::{DataErrors, FakturoidError, UnknownError};
use crate::filters::{Filter, FilterBuilder, InvoiceFilter, SubjectFilter};
use crate::models::{Invoice, InvoiceAction, Subject};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::export::Option::Some;
use serde::Serialize;
use std::collections::HashMap;

pub trait Entity {
    fn url_part() -> &'static str;
    fn filter_builder() -> Box<dyn FilterBuilder>;
}

pub trait Action: ToString {
    fn url_part() -> &'static str;
    fn query(&self) -> HashMap<String, String>;
}

impl Entity for Subject {
    fn url_part() -> &'static str {
        "subjects"
    }

    fn filter_builder() -> Box<dyn FilterBuilder> {
        Box::new(SubjectFilter)
    }
}

impl Entity for Invoice {
    fn url_part() -> &'static str {
        "invoices"
    }

    fn filter_builder() -> Box<dyn FilterBuilder> {
        Box::new(InvoiceFilter)
    }
}

pub struct PagedResponse<T: Entity + DeserializeOwned> {
    collection: Vec<T>,
    client: Fakturoid,
    links: HashMap<String, String>,
}

impl<T: Entity + DeserializeOwned> PagedResponse<T> {
    fn new(collection: Vec<T>, client: Fakturoid, links: HashMap<String, String>) -> Self {
        Self {
            collection,
            client,
            links,
        }
    }

    async fn page(self, page: &str) -> Result<PagedResponse<T>, FakturoidError> {
        if let Some(url) = self.links.get(page) {
            Ok(self.client.get_url(url.as_str(), None).await?)
        } else {
            Ok(self)
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.collection
    }

    pub async fn first_page(self) -> Result<PagedResponse<T>, FakturoidError> {
        Ok(self.page("first").await?)
    }

    pub async fn prev_page(self) -> Result<PagedResponse<T>, FakturoidError> {
        Ok(self.page("prev").await?)
    }

    pub async fn next_page(self) -> Result<PagedResponse<T>, FakturoidError> {
        Ok(self.page("next").await?)
    }

    pub async fn last_page(self) -> Result<PagedResponse<T>, FakturoidError> {
        Ok(self.page("last").await?)
    }

    pub fn has_next(&self) -> bool {
        self.links.contains_key("next")
    }

    pub fn has_prev(&self) -> bool {
        self.links.contains_key("prev")
    }
}

impl Action for InvoiceAction {
    fn url_part() -> &'static str {
        "invoices"
    }

    fn query(&self) -> HashMap<String, String> {
        [("event", self.to_string())]
            .iter()
            .map(|q| (q.0.to_string(), q.1.clone()))
            .collect()
    }
}

#[derive(Clone)]
pub struct Fakturoid {
    user: String,
    password: String,
    slug: String,
    user_agent: Option<String>,
    client: Client,
}

impl Fakturoid {
    pub fn new(user: &str, password: &str, slug: &str, user_agent: Option<&str>) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
            slug: slug.to_string(),
            user_agent: {
                if let Some(ua) = user_agent {
                    Some(ua.to_string())
                } else {
                    None
                }
            },
            client: Client::new(),
        }
    }

    fn url_first(&self) -> String {
        format!("https://app.fakturoid.cz/api/v2/accounts/{}/", self.slug)
    }

    fn url_with_id(&self, entity_part: &str, id: i32) -> String {
        format!("{}{}/{}.json", self.url_first(), entity_part, id)
    }

    fn user_agent(&self) -> String {
        if let Some(ua) = self.user_agent.as_ref() {
            ua.clone()
        } else {
            "Rust API client (pepa@bukova.info)".to_string()
        }
    }

    async fn paged_response<T>(
        &self,
        response: Response,
    ) -> Result<PagedResponse<T>, FakturoidError>
    where
        T: Entity + DeserializeOwned,
    {
        if let Some(link) = response.headers().get("Link") {
            let mut links = HashMap::<String, String>::new();
            for lnk in link
                .to_str()
                .map_err(FakturoidError::from_std_err)?
                .split(",")
            {
                let parts: Vec<_> = lnk.split(";").collect();
                if parts.len() == 2 {
                    let key = parts[1][4..parts[1].len() - 1].trim();
                    let val = parts[0][1..parts[0].len() - 1].trim();
                    links.insert(key.to_string(), val.to_string());
                }
            }
            Ok(PagedResponse::new(
                response.json::<Vec<T>>().await?,
                self.clone(),
                links,
            ))
        } else {
            Ok(PagedResponse::new(
                response.json::<Vec<T>>().await?,
                self.clone(),
                HashMap::<String, String>::new(),
            ))
        }
    }

    async fn get_url<T>(
        &self,
        url: &str,
        filter: Option<HashMap<String, String>>,
    ) -> Result<PagedResponse<T>, FakturoidError>
    where
        T: Entity + DeserializeOwned,
    {
        let resp = if let Some(flt) = filter {
            self.client
                .get(url)
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .query(&flt)
                .send()
                .await?
        } else {
            self.client
                .get(url)
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .send()
                .await?
        };

        self.paged_response(resp).await
    }

    async fn error_response(response: Response) -> FakturoidError {
        if let Err(e) = response.error_for_status_ref() {
            if response.status() == 422 {
                match response.json::<DataErrors>().await {
                    Ok(data) => FakturoidError::from_data(data, e),
                    Err(err) => FakturoidError::from_std_err(err),
                }
            } else {
                e.into()
            }
        } else {
            FakturoidError::from_std_err(UnknownError::new("evaluate_response<T>()"))
        }
    }

    async fn evaluate_response<T>(response: Response) -> Result<T, FakturoidError>
    where
        T: Entity + DeserializeOwned,
    {
        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            Err(Self::error_response(response).await)
        }
    }

    async fn evaluate(response: Response) -> Result<(), FakturoidError> {
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Self::error_response(response).await)
        }
    }

    pub async fn detail<T>(&self, id: i32) -> Result<T, FakturoidError>
    where
        T: Entity + DeserializeOwned,
    {
        Self::evaluate_response(
            self.client
                .get(&self.url_with_id(T::url_part(), id))
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .send()
                .await?,
        )
        .await
    }

    pub async fn update<T>(&self, id: i32, entity: &T) -> Result<T, FakturoidError>
    where
        T: Entity + Serialize + DeserializeOwned,
    {
        Self::evaluate_response(
            self.client
                .patch(&self.url_with_id(T::url_part(), id))
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .json(entity)
                .send()
                .await?,
        )
        .await
    }

    pub async fn delete<T>(&self, id: i32) -> Result<(), FakturoidError>
    where
        T: Entity,
    {
        Self::evaluate(
            self.client
                .delete(&self.url_with_id(T::url_part(), id))
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .send()
                .await?,
        )
        .await
    }

    pub async fn create<T>(&self, entity: &T) -> Result<T, FakturoidError>
    where
        T: Entity + Serialize + DeserializeOwned,
    {
        Self::evaluate_response(
            self.client
                .post(&format!("{}{}.json", self.url_first(), T::url_part()))
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .json(entity)
                .send()
                .await?,
        )
        .await
    }

    pub async fn list<T>(&self, filter: Option<Filter>) -> Result<PagedResponse<T>, FakturoidError>
    where
        T: Entity + DeserializeOwned,
    {
        let filter = if let Some(flt) = filter {
            if !flt.is_empty() {
                Some(T::filter_builder().build(flt))
            } else {
                None
            }
        } else {
            None
        };
        self.get_url(
            format!("{}{}.json", self.url_first(), T::url_part()).as_str(),
            filter,
        )
        .await
    }

    pub async fn fulltext<T>(&self, search: &str) -> Result<PagedResponse<T>, FakturoidError>
    where
        T: Entity + DeserializeOwned,
    {
        let query_map: HashMap<String, String> = [("query".to_string(), search.to_string())]
            .iter()
            .map(|q| (q.0.clone(), q.1.clone()))
            .collect();
        self.get_url(
            format!("{}{}/search.json", self.url_first(), T::url_part()).as_str(),
            Some(query_map),
        )
        .await
    }

    pub async fn action<T: Action, D: Serialize>(
        &self,
        id: i32,
        action: T,
        data: Option<D>,
    ) -> Result<(), FakturoidError> {
        let req = self
            .client
            .post(&format!(
                "{}{}/{}/fire.json",
                self.url_first(),
                T::url_part(),
                id
            ))
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .header("User-Agent", self.user_agent())
            .query(&action.query());
        let req = if let Some(d) = data {
            req.query(&d)
        } else {
            req
        };
        Self::evaluate(req.send().await?).await
    }
}
