use url::{ParseError, Url};
use graphql_client::GraphQLQuery;
use reqwest::blocking::Client;

pub struct APIFetcher {
    path: Url,
    name: String,
}

impl APIFetcher {
    pub fn new(path: Url, name: String) -> APIFetcher {
        APIFetcher { path, name }
    }

    pub fn query_name(&self) -> bool {
        let client = reqwest::blocking::Client::builder()
        .user_agent("graphql-rust/0.9.0")
        .build().unwrap();
        return true;
    }
}
