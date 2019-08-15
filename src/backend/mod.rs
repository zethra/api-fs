mod rest;

use super::{ApiDefinition, FSEndPoint};
use actix::prelude::*;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) enum BackendType {
    REST,
}

// pub(super) fn get_objects_from_api(
//     api_definition: &ApiDefinition,
// ) -> Result<HashMap<u64, FSEndPoint>, Error> {
//     let backend = rest::RestBackend::new(api_definition.clone());
// }

pub(super) fn get_backend(api_definition: &ApiDefinition) -> impl Backend {
    rest::RestBackend::new(api_definition.clone())
}

/// An object in the api backend
pub struct ApiObject {
    /// Path to the api endpoint
    path: PathBuf,
    /// Contents of the object
    contents: String,
}

pub trait Backend: Actor + Handler<GetObjects> where Self: Actor<Context = Context<Self>> {}

// pub trait Backend<ID: AsRef<str>>: Actor + Handler<GetObjects> {
//     type Error;

//     fn get_objects(&self) -> Result<Vec<ApiObject>, Error>;
//     fn create_object(&mut self, object: ApiObject) -> Result<ID, Error>;
//     fn update_object(&mut self, id: ID, object: ApiObject) -> Result<ID, Error>;
//     fn delete_object(&mut self, id: ID) -> Result<ID, Error>;
// }

#[derive(Debug)]
pub struct GetObjects(PathBuf);

impl GetObjects {
    pub fn for_path(path: PathBuf) -> GetObjects {
        GetObjects(path)
    }
}

impl Message for GetObjects {
    type Result = Result<Vec<ApiObject>, Error>;
}
