use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct RootList {
    revision: String,
    length: u32,

    attributes: HashMap<String, Value>,
    timestamp: String,

    contents: RootListContent,
}

#[derive(Serialize, Deserialize)]
struct RootListContent {
    pos: u32,
    truncated: bool,
    items: Vec<RootListItem>,

    #[serde(rename = "metaItems")]
    meta_items: Vec<RootListMetaItem>,
}

#[derive(Serialize, Deserialize)]
struct RootListItem {
    uri: String,
    attributes: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct RootListMetaItem {
    revision: String,
    attributes: HashMap<String, Value>,
    length: u32,
    timestamp: String,

    #[serde(rename = "ownerUsername")]
    owner_username: String,
}
