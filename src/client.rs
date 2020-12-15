use crate::models::Subject;
use serde::Serialize;
use chrono::NaiveDateTime;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::collections::{HashMap, BTreeMap};
use serde::export::Option::Some;
use crate::error::{ApiRequestError, UnknownError};

pub trait Entity {
    fn url_part() -> &'static str;
    fn filter() -> Box<dyn Filter>;
}

pub trait Filter {
    fn params(&self, builder: FilterBuilder) -> BTreeMap<String, String>;
}

#[derive(Default, Clone)]
pub struct FilterBuilder {
    query_map: BTreeMap<String, String>
}

impl FilterBuilder {
    pub fn page(mut self, page: i32) -> Self {
        self.query_map.insert("page".to_string(), format!("{}", page));
        self
    }

    pub fn since(mut self, since: NaiveDateTime) -> Self {
        self.query_map.insert("since".to_string(), since.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    pub fn updated_since(mut self, upd_since: NaiveDateTime) -> Self {
        self.query_map.insert("updated_since".to_string(), upd_since.format("%Y-%m-%dT%H:%M:%S").to_string());
        self
    }

    pub fn custom_id(mut self, custom_id: &str) -> Self {
        self.query_map.insert("custom_id".to_string(), custom_id.to_string());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.query_map.is_empty()
    }
}

#[derive(Default)]
struct SubjectFilter;

impl Filter for SubjectFilter {
    fn params(&self, builder: FilterBuilder) -> BTreeMap<String, String> {
        builder.query_map
    }
}

impl Entity for Subject {
    fn url_part() -> &'static str {
        "subjects"
    }

    fn filter() -> Box<dyn Filter> {
        Box::new(SubjectFilter::default())
    }
}

pub struct PagedResponse<T: Entity + DeserializeOwned> {
    collection: Vec<T>,
    client: Fakturoid,
    links: HashMap<String, String>
}

impl<T: Entity + DeserializeOwned> PagedResponse<T> {
    fn new(collection: Vec<T>,
           client: Fakturoid,
           links: HashMap<String, String>) -> Self {
        Self {
            collection,
            client,
            links
        }
    }

    async fn page(self, page: &str) -> Result<PagedResponse<T>, ApiRequestError> {
        if let Some(url) = self.links.get(page) {
            Ok(self.client.get_url(url.as_str(), None).await?)
        } else {
            Ok(self)
        }
    }

    pub fn data(&self) -> &Vec<T> {
        &self.collection
    }

    pub async fn first_page(self) -> Result<PagedResponse<T>, ApiRequestError> {
        Ok(self.page("first").await?)
    }

    pub async fn prev_page(self) -> Result<PagedResponse<T>, ApiRequestError> {
        Ok(self.page("prev").await?)
    }

    pub async fn next_page(self) -> Result<PagedResponse<T>, ApiRequestError> {
        Ok(self.page("next").await?)
    }

    pub async fn last_page(self) -> Result<PagedResponse<T>, ApiRequestError> {
        Ok(self.page("last").await?)
    }

    pub fn has_next(&self) -> bool {
        self.links.contains_key("next")
    }

    pub fn has_prev(&self) -> bool {
        self.links.contains_key("prev")
    }
}

#[derive(Clone)]
pub struct Fakturoid {
    user: String,
    password: String,
    slug: String,
    user_agent: Option<String>,
    client: Client
}

impl Fakturoid {
    pub fn new(user: &str,
               password: &str,
               slug: &str,
               user_agent: Option<&str>) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
            slug: slug.to_string(),
            user_agent: {if let Some(ua) = user_agent {Some(ua.to_string())} else { None }},
            client: Client::new()
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
            "Rust API client".to_string()
        }
    }

    async fn paged_response<T>(&self, response: Response) -> Result<PagedResponse<T>, ApiRequestError>
        where
            T: Entity + DeserializeOwned
    {
        if let Some(link) = response.headers().get("Link") {
            let mut links = HashMap::<String, String>::new();
            for lnk in link.to_str().map_err(ApiRequestError::from_std_err)?.split(",") {
                let parts: Vec<_> = lnk.split(";").collect();
                if parts.len() == 2 {
                    let key = parts[1][4..parts[1].len() - 1].trim();
                    let val = parts[0][1..parts[0].len() - 1].trim();
                    links.insert(key.to_string(), val.to_string());
                }
            }
            Ok(PagedResponse::new(response.json::<Vec<T>>().await?, self.clone(), links))
        } else {
            Ok(PagedResponse::new(response.json::<Vec<T>>().await?, self.clone(), HashMap::<String, String>::new()))
        }
    }

    async fn get_url<T>(&self, url: &str, filter: Option<BTreeMap<String, String>>) -> Result<PagedResponse<T>, ApiRequestError>
    where
        T: Entity + DeserializeOwned
    {
        let resp = if let Some(flt) = filter {
            self.client.get(url)
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .query(&flt)
                .send().await?
        }else {
            self.client.get(url)
                .basic_auth(self.user.as_str(), Some(self.password.as_str()))
                .header("User-Agent", self.user_agent())
                .send().await?
        };

        Ok(self.paged_response(resp).await?)
    }

    pub async fn detail<T>(&self, id: i32) -> Result<T, ApiRequestError>
    where
        T: Entity + DeserializeOwned
    {
        let resp = self.client.get(&self.url_with_id(T::url_part(), id))
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .header("User-Agent", self.user_agent())
            .send().await?
            .json::<T>().await?;
        Ok(resp)
    }

    pub async fn update<T>(&self, id: i32, entity: &T) -> Result<T, ApiRequestError>
    where
        T: Entity + Serialize + DeserializeOwned
    {
        let send_resp = self.client.patch(&self.url_with_id(T::url_part(), id))
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .header("User-Agent", self.user_agent())
            .json(entity)
            .send().await?;
        if send_resp.status().is_success() {
            Ok(send_resp.json::<T>().await?)
        } else {
            let err = if let Err(e) = send_resp.error_for_status_ref() {
                ApiRequestError::from_data(send_resp.json().await?, e)
            } else {
                ApiRequestError::from_std_err(UnknownError::new("update<T>()"))
            };

            Err(err)
        }
    }

    pub async fn delete<T>(&self, id: i32) -> Result<(), ApiRequestError>
    where
        T: Entity
    {
        self.client.delete(&self.url_with_id(T::url_part(), id))
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .header("User-Agent", self.user_agent())
            .send().await?;
        Ok(())
    }

    pub async fn create<T>(&self, entity: &T) -> Result<T, ApiRequestError>
    where
        T: Entity + Serialize + DeserializeOwned
    {
        let resp = self.client.post(&format!("{}{}.json", self.url_first(), T::url_part()))
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .header("User-Agent", self.user_agent())
            .json(entity)
            .send().await?
            .json::<T>().await?;
        Ok(resp)
    }

    pub async fn list<T>(&self, filter_builder: Option<FilterBuilder>) -> Result<PagedResponse<T>, ApiRequestError>
    where
        T: Entity + DeserializeOwned
    {
        let filter = if let Some(builder) = filter_builder {
            if !builder.is_empty() {
                Some(T::filter().params(builder))
            } else { None }
        } else { None };
        Ok(self.get_url(format!("{}{}.json", self.url_first(), T::url_part()).as_str(), filter).await?)
    }
}