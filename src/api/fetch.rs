use crate::CONFIG_FILE_PATH;
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::fs;
use url::Url;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct RegisterMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct VerifyMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct AuthMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct RefreshMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct SubgroupsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct UserQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct ThreadUpdateMutation;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct ThreadsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/resources/schema.gql",
    query_path = "src/api/resources/queries.gql"
)]
struct ThreadContentQuery;

#[derive(Debug, Clone)]
pub struct APIFetcher {
    path: Url,
    client: Client,
    token: String,
    node_id: String,
}

impl APIFetcher {
    pub fn new(path: Url, top_id: String) -> APIFetcher {
        let client = Client::builder()
            .user_agent("borz_client/0.1.0")
            .build()
            .unwrap();
        APIFetcher {
            path,
            client,
            token: String::new(),
            node_id: top_id,
        }
    }

    pub fn child(&mut self, id: String) -> APIFetcher {
        let res = self.query_subgroups();
        for subgroup in res.data.unwrap().subgroup.unwrap().child_group.edges {
            let node = subgroup.unwrap().node.unwrap();
            if id == node.id {
                return APIFetcher {
                    path: self.path.clone(),
                    client: Client::builder()
                        .user_agent("borz_client/0.1.0")
                        .build()
                        .unwrap(),
                    token: String::new(),
                    node_id: id,
                };
            }
        }
        panic!("Unfound child");
    }

    pub fn mutate_register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Response<register_mutation::ResponseData> {
        let mutation = RegisterMutation::build_query(register_mutation::Variables {
            email,
            username,
            password,
        });
        let res = self
            .client
            .post(self.path.clone())
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn mutate_verify(&self, token: String) -> Response<verify_mutation::ResponseData> {
        let mutation = VerifyMutation::build_query(verify_mutation::Variables { token });
        let res = self
            .client
            .post(self.path.clone())
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn mutate_refresh(&mut self) {
        let content = fs::read_to_string(CONFIG_FILE_PATH.clone()).unwrap();
        let json = json::parse(&content[..]).unwrap();
        let refresh = json["refresh_token"].to_string();
        let username = json["username"].to_string();
        let mutation = RefreshMutation::build_query(refresh_mutation::Variables {
            refresh_token: refresh,
        });
        let res = self
            .client
            .post(self.path.clone())
            .json(&mutation)
            .send()
            .unwrap();
        let body: Response<refresh_mutation::ResponseData> = res.json().unwrap();
        let body = body.data.unwrap();
        let rt = body.refresh_token.unwrap();
        if !rt.success.unwrap() {
            panic!("Unsuccessful parse");
        }
        self.token = rt.token.unwrap();
        let refresh_token = rt.refresh_token.unwrap();
        fs::write(
            CONFIG_FILE_PATH.clone(),
            format!(
                "{{\"token\": \"{}\", \"refresh_token\": \"{}\", \"username\": \"{}\"}}",
                self.token, refresh_token, username
            ),
        )
        .unwrap();
    }

    pub fn query_user(&mut self, uid: String) -> Response<user_query::ResponseData> {
        self.mutate_refresh();
        let mutation = UserQuery::build_query(user_query::Variables { id: uid });
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("JWT {}", self.token).parse().unwrap(),
        );
        let res = self
            .client
            .post(self.path.clone())
            .headers(headers)
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn query_thread_content(
        &mut self,
        tid: String,
    ) -> Response<thread_content_query::ResponseData> {
        self.mutate_refresh();
        let mutation = ThreadContentQuery::build_query(thread_content_query::Variables { id: tid });
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("JWT {}", self.token).parse().unwrap(),
        );
        let res = self
            .client
            .post(self.path.clone())
            .headers(headers)
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn query_threads(&mut self) -> Response<threads_query::ResponseData> {
        self.mutate_refresh();
        let mutation = ThreadsQuery::build_query(threads_query::Variables {
            id: self.node_id.clone(),
        });
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("JWT {}", self.token).parse().unwrap(),
        );
        let res = self
            .client
            .post(self.path.clone())
            .headers(headers)
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn query_subgroups(&mut self) -> Response<subgroups_query::ResponseData> {
        self.mutate_refresh();
        let mutation = SubgroupsQuery::build_query(subgroups_query::Variables {
            id: self.node_id.clone(),
        });
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("JWT {}", self.token).parse().unwrap(),
        );
        let res = self
            .client
            .post(self.path.clone())
            .headers(headers)
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn mutate_thread_reply(
        &mut self,
        id: String,
        content: String,
    ) -> Response<thread_update_mutation::ResponseData> {
        self.mutate_refresh();
        let mutation =
            ThreadUpdateMutation::build_query(thread_update_mutation::Variables { id, content });
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("JWT {}", self.token).parse().unwrap(),
        );
        let res = self
            .client
            .post(self.path.clone())
            .headers(headers)
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }

    pub fn mutate_auth(
        &mut self,
        username: String,
        password: String,
    ) -> Response<auth_mutation::ResponseData> {
        let mutation = AuthMutation::build_query(auth_mutation::Variables { username, password });
        let res = self
            .client
            .post(self.path.clone())
            .json(&mutation)
            .send()
            .unwrap();
        return res.json().unwrap();
    }
}
