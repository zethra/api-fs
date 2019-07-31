mod rest;

use super::{ApiDefinition, FSEndPoint};
use std::collections::HashMap;
use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum BackendType {
    REST,
}

pub(super) fn get_objects_from_api(api_definition: &ApiDefinition) -> Result<HashMap<u64, FSEndPoint>, Error> {
    unimplemented!();
}