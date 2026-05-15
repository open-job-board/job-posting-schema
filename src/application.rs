use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// How to apply for the role.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    /// An email address to which applications can be sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Name and/or role of a contact person at the hiring organization
    /// (e.g. "Jane Doe — Talent Acquisition").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,

    /// A URL to the application form or applicant tracking system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Last date on which applications are accepted. Maps to schema.org's
    /// `validThrough` and is useful for deprioritizing or archiving
    /// expiring postings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<NaiveDate>,
}
