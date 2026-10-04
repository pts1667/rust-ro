use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::status_change::StatusChangeKind;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StatusMetadata {
    pub icon: Option<String>,
    pub duration_lookup: Option<String>,
    #[serde(default)]
    pub states: BTreeMap<String, bool>,
    #[serde(default)]
    pub flags: BTreeMap<String, bool>,
    #[serde(default)]
    pub fail: BTreeMap<String, bool>,
    #[serde(default)]
    pub end_on_start: BTreeMap<String, bool>,
    #[serde(default)]
    pub end_return: BTreeMap<String, bool>,
    #[serde(default)]
    pub end_on_end: BTreeMap<String, bool>,
    #[serde(default)]
    pub min_rate: i32,
    pub min_duration: Option<i32>,
}

impl StatusChangeKind {
    pub fn metadata(self) -> &'static StatusMetadata {
        static METADATA: OnceLock<BTreeMap<String, StatusMetadata>> = OnceLock::new();
        METADATA.get_or_init(|| serde_json::from_str(include_str!("status_change_metadata.json")).expect("Embedded pre-renewal status metadata is invalid"))
            .get(self.name().strip_prefix("SC_").unwrap()).expect("Status change lacks pre-renewal metadata")
    }

    pub fn dispellable(self) -> bool { !self.metadata().flags.get("NoDispell").copied().unwrap_or(false) }
    pub fn removed_by_death(self) -> bool { !self.metadata().flags.get("NoRemoveOnDead").copied().unwrap_or(false) }
}
