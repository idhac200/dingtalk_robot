use serde::Serialize;
use std::collections::HashSet;
use std::ops::Not;

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct At {
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    at_mobiles: HashSet<String>,
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    at_user_ids: HashSet<String>,
    #[serde(skip_serializing_if = "Not::not")]
    is_at_all: bool,
}

impl At {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_mobile(mut self, mobile: impl Into<String>) -> Self {
        self.at_mobiles.insert(mobile.into());
        self
    }

    pub fn add_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.at_user_ids.insert(user_id.into());
        self
    }

    pub fn set_at_all(mut self) -> Self {
        self.is_at_all = true;
        self
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.at_mobiles.is_empty() && self.at_user_ids.is_empty() && self.is_at_all.not()
    }
}
