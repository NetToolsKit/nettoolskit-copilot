//! Validation profile loading for `validate-all`.

use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ValidationProfilesDocument {
    #[serde(rename = "defaultProfile")]
    pub default_profile: Option<String>,
    #[serde(default)]
    pub profiles: Vec<ValidationProfile>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ValidationProfile {
    pub id: String,
    #[serde(rename = "warningOnly", default)]
    pub warning_only: bool,
    #[serde(rename = "checkOrder", default)]
    pub check_order: Vec<String>,
    #[serde(rename = "checkOptions", default)]
    pub check_options: BTreeMap<String, Map<String, Value>>,
}

pub(crate) fn load_profiles_document(
    profiles_path: &Path,
    suite_warnings: &mut Vec<String>,
) -> Option<ValidationProfilesDocument> {
    if !profiles_path.is_file() {
        suite_warnings.push(format!(
            "Missing validation profiles: {}",
            profiles_path.display()
        ));
        return None;
    }

    let document = match fs::read_to_string(profiles_path) {
        Ok(document) => document,
        Err(error) => {
            suite_warnings.push(format!(
                "Could not read validation profiles '{}': {}",
                profiles_path.display(),
                error
            ));
            return None;
        }
    };

    match serde_json::from_str::<ValidationProfilesDocument>(&document) {
        Ok(parsed) => Some(parsed),
        Err(error) => {
            suite_warnings.push(format!(
                "Invalid JSON in validation profiles '{}': {}",
                profiles_path.display(),
                error
            ));
            None
        }
    }
}

pub(crate) fn select_profile(
    document: Option<&ValidationProfilesDocument>,
    requested_profile_id: Option<&str>,
    suite_warnings: &mut Vec<String>,
) -> Option<ValidationProfile> {
    let document = document?;

    let selected_profile_id = requested_profile_id
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .or_else(|| document.default_profile.clone());

    let Some(selected_profile_id) = selected_profile_id else {
        suite_warnings.push("Validation profiles document has no defaultProfile.".to_string());
        return None;
    };

    let selected_profile = document
        .profiles
        .iter()
        .find(|profile| profile.id == selected_profile_id)
        .cloned();

    if selected_profile.is_none() {
        suite_warnings.push(format!(
            "Validation profile not found: {selected_profile_id}"
        ));
    }

    selected_profile
}