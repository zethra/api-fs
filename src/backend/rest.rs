use super::*;
use crate::ApiDefinition;
use actix::prelude::*;
use failure::err_msg;
use futures::{future, Future, Stream};
use reqwest::r#async::{Client, Decoder};
use serde_json::{self, Value};
// use std::io::{self, Cursor};
use std::mem;
use try_future::*;

#[derive(Debug)]
struct RestBackend {
    api_definition: ApiDefinition,
    client: Client,
}

impl Actor for RestBackend {
    type Context = Context<Self>;
}

impl Handler<GetObjects> for RestBackend {
    type Result = Box<Future<Item = Vec<ApiObject>, Error = Error>>;

    fn handle(&mut self, msg: GetObjects, _ctx: &mut Context<Self>) -> Self::Result {
        let GetObjects(path) = msg;
        let path_str = try_future_box!(path.to_str().ok_or(err_msg("Path is not a valid string")));
        let mut url = self.api_definition.url.clone();
        url.push_str(&path_str);
        Box::from(
            Client::new()
                .get(&url)
                .send()
                .and_then(|mut res| {
                    println!("{}", res.status());
                    let body = mem::replace(res.body_mut(), Decoder::empty());
                    body.concat2()
                })
                .map_err(|err| err.into())
                .and_then(move |body| {
                    let mut json: Value = try_future_box!(serde_json::from_slice(&body));
                    Box::new(future::ok(match json.as_array_mut() {
                        Some(array) => array.into_iter().fold(Vec::new(), |mut acc, item| {
                            let item = mem::replace(item, Value::Null);
                            let item_str = serde_json::from_value(item)
                                .expect("Serialization of deserialize value failed");
                            acc.push(ApiObject {
                                path: path.clone(),
                                contents: item_str,
                            });
                            acc
                        }),
                        None => {
                            return Box::new(future::err(err_msg("Base object not an array")));
                        }
                    }))
                }),
        )
    }
}
