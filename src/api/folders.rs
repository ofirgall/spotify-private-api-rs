use std::collections::HashMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
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
    revision: Option<String>,
    attributes: Option<HashMap<String, Value>>,
    length: Option<u32>,
    timestamp: Option<String>,

    #[serde(rename = "ownerUsername")]
    owner_username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Changes {
    #[serde(rename = "baseRevision")]
    base_revision: String,
    deltas: Vec<Delta>,

    // Unknown Features
    #[serde(rename = "wantResultingRevisions")]
    want_resulting_revisions: bool,
    #[serde(rename = "wantSyncResult")]
    want_sync_result: bool,
    nonces: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delta {
    ops: Vec<Operation>,
    info: DeltaInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeltaInfo {
    user: String,
    timestamp: String,
    admin: bool,
    undo: bool,
    redo: bool,
    merge: bool,
    compressed: bool,
    migration: bool,
    #[serde(rename = "splitId")]
    split_id: i32,

    source: DeltaInfoSource,
}

impl Default for DeltaInfo {
    fn default() -> Self {
        Self {
            user: "".to_string(),
            timestamp: "0".to_string(),
            admin: false,
            undo: false,
            redo: false,
            merge: false,
            compressed: false,
            migration: false,
            split_id: 0i32,
            source: DeltaInfoSource::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DeltaInfoSource {
    client: String,
    app: String,
    source: String,
    version: String,
}

impl Default for DeltaInfoSource {
    fn default() -> Self {
        Self {
            client: "WEBPLAYER".to_string(),
            app: "".to_string(),
            source: "".to_string(),
            version: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind")]
enum Operation {
    #[serde(rename = "ADD")]
    Add(AddOperation),
    #[serde(rename = "REM")]
    Rem(RemoveOperation),
    #[serde(rename = "MOV")]
    Mov(MoveOperation),
}

#[derive(Serialize, Deserialize, Debug)]
struct AddOperation {
    add: AddOperationParams,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddOperationParams {
    #[serde(rename = "fromIndex")]
    from_index: u32,

    items: Vec<OperationItem>,

    // Unknown Features
    #[serde(rename = "addLast")]
    add_last: bool,
    #[serde(rename = "addFirst")]
    add_first: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveOperation {
    rem: RemoveOperationParams,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveOperationParams {
    #[serde(rename = "fromIndex")]
    from_index: u32,
    length: u32,

    // Unknown Features
    items: Vec<Value>,
    #[serde(rename = "itemsAsKey")]
    items_as_key: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct MoveOperation {
    rem: MoveOperationParams,
}

#[derive(Serialize, Deserialize, Debug)]
struct MoveOperationParams {
    #[serde(rename = "fromIndex")]
    from_index: u32,

    #[serde(rename = "toIndex")]
    to_index: u32,

    length: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct OperationItem {
    uri: String,

    attributes: OperationItemAttrs,
}

impl OperationItem {
    fn new_playlist(uri: String) -> Self {
        Self {
            uri: format!("spotify:playlist:{}", uri),
            attributes: OperationItemAttrs::default(),
        }
    }

    fn new_start_folder(uri: String, folder_name: &str) -> Self {
        // TODO: generate uri?
        Self {
            uri: format!("spotify:start-group:{}:{}", uri, folder_name),
            attributes: OperationItemAttrs::default(),
        }
    }

    fn new_end_folder(uri: String) -> Self {
        Self {
            uri: format!("spotify:end-group:{}", uri),
            attributes: OperationItemAttrs::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct OperationItemAttrs {
    #[serde(rename = "addedBy")]
    added_by: String,

    timestamp: String,

    #[serde(rename = "seenAt")]
    seen_at: String,

    public: bool,

    #[serde(rename = "formatAttributes")]
    format_attributes: Vec<Value>,
}

impl Default for OperationItemAttrs {
    fn default() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            added_by: "".to_string(),
            timestamp: timestamp.as_millis().to_string(),
            seen_at: "0".to_string(),
            public: false,
            format_attributes: vec![],
        }
    }
}

// TODO: tmp function to test structs
pub fn add_folder(revision: &str, name: &str, start_index: u32, end_index: u32) -> Changes {
    Changes {
        base_revision: revision.to_string(),
        want_resulting_revisions: false,
        want_sync_result: false,
        nonces: vec![],
        deltas: vec![Delta {
            ops: vec![
                Operation::Add(AddOperation {
                    add: AddOperationParams {
                        from_index: start_index,
                        items: vec![OperationItem::new_start_folder(
                            "123456789abcdefa".to_string(),
                            name,
                        )],
                        add_last: false,
                        add_first: false,
                    },
                }),
                Operation::Add(AddOperation {
                    add: AddOperationParams {
                        from_index: end_index,
                        items: vec![OperationItem::new_end_folder(
                            "123456789abcdefa".to_string(),
                        )],
                        add_last: false,
                        add_first: false,
                    },
                }),
            ],
            info: DeltaInfo::default(),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::RootList;

    #[test]
    fn test_root_list_parse() {
        let api_response = r#"{"revision":"AAAACf20+ElrZP4No2PQNHbgGYa/ht3r","length":4,"attributes":{},"contents":{"pos":0,"truncated":false,"items":[{"uri":"spotify:start-group:123456789abcdefa:Abablagan","attributes":{"timestamp":"1665495078416","seenAt":"0","public":false}},{"uri":"spotify:end-group:123456789abcdefa","attributes":{"timestamp":"1665495078416","seenAt":"0","public":false}},{"uri":"spotify:playlist:5aNzxEEkRE9MgNkiuXmpOR","attributes":{"timestamp":"1665486971754","seenAt":"0","public":false}},{"uri":"spotify:playlist:3FKTkhbClLGgKdPpbx3aHy","attributes":{"timestamp":"1665486908663","seenAt":"0","public":false}}],"metaItems":[{},{},{"revision":"AAAAAX9FIoTlMkv9e4zCryuZtD/yioLv","attributes":{"name":"My Playlist #2"},"length":0,"timestamp":"1665486971670","ownerUsername":"31h5mfzvglpwfevvaens2flw7smu"},{"revision":"AAAAAvZixvi5cLYefOMaVOKtGZUJS5pE","attributes":{"name":"My Playlist #1"},"length":1,"timestamp":"1665486922515","ownerUsername":"31h5mfzvglpwfevvaens2flw7smu"}]},"timestamp":"1665495078416"}"#;

        let rl: RootList = serde_json::from_str(api_response).unwrap();

        assert_eq!(rl.revision, "AAAACf20+ElrZP4No2PQNHbgGYa/ht3r")
    }
}
