use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct RootList {
    pub revision: String,
    length: u32,

    attributes: HashMap<String, Value>,
    timestamp: String,

    contents: RootListContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct RootListContent {
    pos: u32,
    truncated: bool,
    items: Vec<RootListItem>,

    #[serde(rename = "metaItems")]
    meta_items: Vec<RootListMetaItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RootListItem {
    uri: String,
    attributes: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RootListMetaItem {
    revision: String,
    attributes: HashMap<String, Value>,
    length: u32,
    timestamp: String,

    #[serde(rename = "ownerUsername")]
    owner_username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Changes {
    #[serde(rename = "baseRevision")]
    base_revision: String,

    #[serde(rename = "wantResultingRevisions")]
    want_resulting_revisions: bool,

    #[serde(rename = "wantSyncResult")]
    want_sync_result: bool,

    nonces: Vec<Value>,

    deltas: Vec<Delta>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delta {
    ops: Vec<Operation>,
    info: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
enum Operation {
    #[serde(rename = "ADD")]
    Add(AddOperation),
}

#[derive(Serialize, Deserialize, Debug)]
struct AddOperation {
    #[serde(skip_serializing)]
    kind: String,
    add: AddOperationParams,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddOperationParams {
    #[serde(rename = "fromIndex")]
    from_index: u32,

    items: Vec<OperationItem>,

    #[serde(rename = "addLast")]
    add_last: bool,

    #[serde(rename = "addFirst")]
    add_first: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OperationItem {
    uri: String,
    attributes: Value,
}

// TODO: tmp function to test structs
pub fn add_folder(revision: &str, name: &str, start_index: u32, end_index: u32) -> Changes {
    let op_item_attrs = json!({
        "addedBy": "",
        "timestamp": "1665585275953",
        "seenAt": "0",
        "public": false,
        "formatAttributes": []
    });
    Changes {
        base_revision: revision.to_string(),
        want_resulting_revisions: false,
        want_sync_result: false,
        nonces: vec![],
        deltas: vec![Delta {
            ops: vec![
                Operation::Add(AddOperation {
                    kind: "ADD".to_string(),
                    add: AddOperationParams {
                        from_index: start_index,
                        items: vec![
                            OperationItem {
                                uri: format!("spotify:start-group:123456789abcdefa:{}", name),
                                attributes: op_item_attrs.clone(),
                            },
                            OperationItem {
                                uri: "spotify:end-group:123456789abcdefa".to_string(),
                                attributes: op_item_attrs,
                            },
                        ],
                        add_last: false,
                        add_first: false,
                    },
                }),
                // Operation::Add(AddOperation {
                //     kind: "ADD".to_string(),
                //     add: AddOperationParams {
                //         from_index: end_index,
                //         items: vec![OperationItem {
                //             uri: format!("spotify:end-group:123456789abcdefa:{}", name),
                //             attributes: op_item_attrs,
                //         }],
                //         add_last: false,
                //         add_first: false,
                //     },
                // }),
            ],
            info: json!({
                "user": "",
                "timestamp": "0",
                "admin": false,
                "undo": false,
                "redo": false,
                "merge": false,
                "compressed": false,
                "migration": false,
                "splitId": 0i32,
                "source": {
                    "client": "WEBPLAYER",
                    "app": "",
                    "source": "",
                    "version": ""
                }
            }),
        }],
    }
}
