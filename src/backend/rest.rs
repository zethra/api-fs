use super::*;
use crate::ApiDefinition;
use actix::prelude::*;
use reqwest::Client;

#[derive(Debug)]
struct RestBackend {
    api_definition: ApiDefinition,
    client: Client,
}

impl Actor for RestBackend {
    type Context = Context<Self>;
}

impl Handler<GetObjects> for RestBackend {
    type Result = Result<Vec<ApiObject>, Error>;

    fn handle(&mut self, _: GetObjects, ctx: &mut Context<Self>) -> Self::Result {
        self.api_definition.endpoints.iter().map(|ep| {
            let url = format!("{}/{}", self.api_definition.url, ep);
            self.client.get(&url)
        });
        Ok(Vec::new())
    }
}
