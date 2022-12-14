use std::cell::RefCell;
use std::collections::HashMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(not(test))]
fn now() -> SystemTime {
    SystemTime::now()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RootList {
    pub revision: String,
    length: u32,

    attributes: HashMap<String, Value>,
    timestamp: String,

    contents: RootListContent,

    #[serde(skip)]
    generated_uris: RefCell<Vec<String>>,
}

impl RootList {
    pub fn generate_folder_uri(&self) -> String {
        let generated_uri = {
            let generated_uris_borrowed = self.generated_uris.borrow();

            let all_uris: Vec<&String> = self
                .contents
                .items
                .iter()
                .map(|x| &x.uri)
                .chain(generated_uris_borrowed.iter())
                .collect();

            generate_folder_uri(all_uris)
        };

        self.generated_uris.borrow_mut().push(generated_uri.clone());

        generated_uri
    }

    pub fn new_request(&self) -> FolderRequest {
        FolderRequest::new(&self.revision)
    }
}

const URI_LENGTH: usize = 16;
const URI_CHARSET: &str = "abcdef1234567890";

fn generate_folder_uri(uris: Vec<&String>) -> String {
    loop {
        let rand_uri = random_string::generate(URI_LENGTH, URI_CHARSET);

        if !uris.contains(&&rand_uri) {
            return rand_uri;
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RootListContent {
    pos: u32,
    truncated: bool,
    items: Vec<RootListItem>,

    #[serde(rename = "metaItems")]
    meta_items: Vec<RootListMetaItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RootListItem {
    uri: String,
    attributes: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RootListMetaItem {
    revision: Option<String>,
    attributes: Option<HashMap<String, Value>>,
    length: Option<u32>,
    timestamp: Option<String>,

    #[serde(rename = "ownerUsername")]
    owner_username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

impl Default for Changes {
    fn default() -> Self {
        Self {
            base_revision: "".to_string(),
            deltas: vec![],
            want_resulting_revisions: false,
            want_sync_result: false,
            nonces: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct Delta {
    ops: Vec<Operation>,
    info: DeltaInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "kind")]
enum Operation {
    #[serde(rename = "ADD")]
    Add(AddOperation),
    #[serde(rename = "REM")]
    Rem(RemoveOperation),
    #[serde(rename = "MOV")]
    Mov(MoveOperation),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct AddOperation {
    add: AddOperationParams,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct RemoveOperation {
    rem: RemoveOperationParams,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
struct RemoveOperationParams {
    #[serde(rename = "fromIndex")]
    from_index: u32,
    length: u32,

    // Unknown Features
    items: Vec<Value>,
    #[serde(rename = "itemsAsKey")]
    items_as_key: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MoveOperation {
    mov: MoveOperationParams,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct MoveOperationParams {
    #[serde(rename = "fromIndex")]
    from_index: u32,

    #[serde(rename = "toIndex")]
    to_index: u32,

    length: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct OperationItem {
    uri: String,

    attributes: OperationItemAttrs,
}

impl OperationItem {
    #[allow(dead_code)] // TODO: remove this after we test it
    fn new_playlist(uri: String) -> Self {
        Self {
            uri: format!("spotify:playlist:{}", uri),
            attributes: OperationItemAttrs::default(),
        }
    }

    fn new_start_folder(uri: &str, folder_name: &str) -> Self {
        Self {
            uri: format!("spotify:start-group:{}:{}", uri, folder_name),
            attributes: OperationItemAttrs::default(),
        }
    }

    fn new_end_folder(uri: &str) -> Self {
        Self {
            uri: format!("spotify:end-group:{}", uri),
            attributes: OperationItemAttrs::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        let timestamp = now()
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

/// Build a changes request
pub struct FolderRequest {
    revision: String,
    ops: Vec<Operation>,
}

impl FolderRequest {
    pub fn new(revision: &str) -> Self {
        Self {
            revision: revision.to_string(),
            ops: vec![],
        }
    }

    pub fn build(&self) -> Changes {
        Changes {
            base_revision: self.revision.clone(),
            deltas: vec![Delta {
                ops: self.ops.clone(),
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    pub fn add(&mut self, name: &str, uri: &str, start_index: u32, end_index: u32) -> &Self {
        self.ops.push(Operation::Add(AddOperation {
            add: AddOperationParams {
                from_index: start_index,
                items: vec![OperationItem::new_start_folder(uri, name)],
                ..Default::default()
            },
        }));

        self.ops.push(Operation::Add(AddOperation {
            add: AddOperationParams {
                from_index: end_index,
                items: vec![OperationItem::new_end_folder(uri)],
                ..Default::default()
            },
        }));

        self
    }

    pub fn remove(&mut self, start_index: u32, length: u32) -> &Self {
        self.ops.push(Operation::Rem(RemoveOperation {
            rem: RemoveOperationParams {
                from_index: start_index,
                length,
                ..Default::default()
            },
        }));

        self
    }

    pub fn mov(&mut self, from_index: u32, to_index: u32, length: u32) -> &Self {
        self.ops.push(Operation::Mov(MoveOperation {
            mov: MoveOperationParams {
                from_index,
                to_index,
                length,
            },
        }));

        self
    }
}

#[cfg(test)]
mod tests {

    use crate::api::folders::mock_time::set_mock_time;

    use super::{generate_folder_uri, FolderRequest, RootList};

    const REV: &str = "AAAAELqqrKuzaoeUKYP7gEzCzrx3h0rD";

    #[test]
    fn test_root_list_des() {
        let api_response = r#"{"revision":"AAAAELqqrKuzaoeUKYP7gEzCzrx3h0rD","length":4,"attributes":{},"contents":{"pos":0,"truncated":false,"items":[{"uri":"spotify:start-group:123456789abcdefa:Abablagan","attributes":{"timestamp":"1665495078416","seenAt":"0","public":false}},{"uri":"spotify:end-group:123456789abcdefa","attributes":{"timestamp":"1665495078416","seenAt":"0","public":false}},{"uri":"spotify:playlist:5aNzxEEkRE9MgNkiuXmpOR","attributes":{"timestamp":"1665486971754","seenAt":"0","public":false}},{"uri":"spotify:playlist:3FKTkhbClLGgKdPpbx3aHy","attributes":{"timestamp":"1665486908663","seenAt":"0","public":false}}],"metaItems":[{},{},{"revision":"AAAAAX9FIoTlMkv9e4zCryuZtD/yioLv","attributes":{"name":"My Playlist #2"},"length":0,"timestamp":"1665486971670","ownerUsername":"31h5mfzvglpwfevvaens2flw7smu"},{"revision":"AAAAAvZixvi5cLYefOMaVOKtGZUJS5pE","attributes":{"name":"My Playlist #1"},"length":1,"timestamp":"1665486922515","ownerUsername":"31h5mfzvglpwfevvaens2flw7smu"}]},"timestamp":"1665495078416"}"#;

        let rl: RootList = serde_json::from_str(api_response).expect("Couldn't parse rootlist");

        assert_eq!(rl.revision, REV);
        rl.generate_folder_uri();
    }

    #[test]
    fn test_root_list_ser() {
        // TODO: Implement
    }

    #[test]
    fn test_add_des() {
        // TODO: Implement
    }

    #[test]
    fn test_add_ser() {
        set_mock_time(1665582465479);

        let changes = FolderRequest::new(REV)
            .add("TestFolder", "123456789abcdefa", 0, 2)
            .build();

        let expected = serde_json::from_str(r#"{"baseRevision":"AAAAELqqrKuzaoeUKYP7gEzCzrx3h0rD","deltas":[{"ops":[{"kind":"ADD","add":{"fromIndex":0,"items":[{"uri":"spotify:start-group:123456789abcdefa:TestFolder","attributes":{"addedBy":"","timestamp":"1665582465479","seenAt":"0","public":false,"formatAttributes":[]}}],"addLast":false,"addFirst":false}},{"kind":"ADD","add":{"fromIndex":2,"items":[{"uri":"spotify:end-group:123456789abcdefa","attributes":{"addedBy":"","timestamp":"1665582465479","seenAt":"0","public":false,"formatAttributes":[]}}],"addLast":false,"addFirst":false}}],"info":{"user":"","timestamp":"0","admin":false,"undo":false,"redo":false,"merge":false,"compressed":false,"migration":false,"splitId":0,"source":{"client":"WEBPLAYER","app":"","source":"","version":""}}}],"wantResultingRevisions":false,"wantSyncResult":false,"nonces":[]}"#).expect("Coudln't parse expected json");

        assert_eq!(changes, expected);
    }

    #[test]
    fn test_rem_des() {
        // TODO: Implement
    }

    #[test]
    fn test_rem_ser() {
        let changes = FolderRequest::new(REV).remove(23, 2).build();

        let expected = serde_json::from_str(r#"{"baseRevision":"AAAAELqqrKuzaoeUKYP7gEzCzrx3h0rD","deltas":[{"ops":[{"kind":"REM","rem":{"fromIndex":23,"length":2,"items":[],"itemsAsKey":false}}],"info":{"user":"","timestamp":"0","admin":false,"undo":false,"redo":false,"merge":false,"compressed":false,"migration":false,"splitId":0,"source":{"client":"WEBPLAYER","app":"","source":"","version":""}}}],"wantResultingRevisions":false,"wantSyncResult":false,"nonces":[]}"#).expect("Coudln't parse expected json");

        assert_eq!(changes, expected);
    }

    #[test]
    fn test_mov_des() {
        // TODO: Implement
    }

    #[test]
    fn test_mov_ser() {
        let changes = FolderRequest::new(REV).mov(6, 8, 1).build();

        let expected = serde_json::from_str(r#"{"baseRevision":"AAAAELqqrKuzaoeUKYP7gEzCzrx3h0rD","deltas":[{"ops":[{"kind":"MOV","mov":{"fromIndex":6,"length":1,"toIndex":8}}],"info":{"user":"","timestamp":"0","admin":false,"undo":false,"redo":false,"merge":false,"compressed":false,"migration":false,"splitId":0,"source":{"client":"WEBPLAYER","app":"","source":"","version":""}}}],"wantResultingRevisions":false,"wantSyncResult":false,"nonces":[]}"#).expect("Coudln't parse expected json");

        assert_eq!(changes, expected);
    }

    #[test]
    fn test_gen_uri() {
        generate_folder_uri(vec![&"123456789abcdefa".to_string()]);
    }
}

#[cfg(test)]
pub mod mock_time {
    use super::*;
    use std::{cell::RefCell, time::Duration};

    thread_local! {
        static MOCK_TIME: RefCell<Option<SystemTime>> = RefCell::new(None);
    }

    pub fn now() -> SystemTime {
        MOCK_TIME.with(|cell| {
            cell.borrow()
                .as_ref()
                .cloned()
                .unwrap_or_else(SystemTime::now)
        })
    }

    pub fn set_mock_time(epoch: u64) {
        let time: SystemTime = SystemTime::UNIX_EPOCH
            .checked_add(Duration::from_millis(epoch))
            .expect("couldn't create time");
        MOCK_TIME.with(|cell| *cell.borrow_mut() = Some(time));
    }

    pub fn clear_mock_time() {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = None);
    }
}

#[cfg(test)]
pub use mock_time::now;
