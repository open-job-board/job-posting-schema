use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::industry::Industry;

/// Expected work setup. `onsite` requires full physical presence; `hybrid`
/// mixes on-site and remote days (see `contract.telecommuteDaysPerWeek`);
/// `remote` allows fully remote work with no required on-site presence.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum WorkMode {
    Onsite,
    Hybrid,
    Remote,
}

/// The role itself — what the job is about and where it sits in the organization.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    /// The plain job title as it would appear at the top of the posting
    /// (e.g. "Senior Backend Engineer").
    pub title: String,

    /// The expected work setup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_mode: Option<WorkMode>,

    /// The team, department, or business unit the role belongs to
    /// (e.g. "Platform Engineering", "Marketing — Brand").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,

    /// The primary industry this specific role is associated with. May differ
    /// from the hiring organization's `industries` (e.g. an industrial company
    /// hiring for its software division).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<Industry>,

    /// A short narrative explaining why this role exists right now — a new
    /// project, a growth phase, a replacement, etc. Helps the applicant
    /// understand the motivation behind the posting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// A detailed description of the role's purpose and business value, the
    /// team size and composition, the working conditions, and what success
    /// in the role looks like.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,

    /// The day-to-day duties and ownership areas expected of the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsibilities: Option<Vec<String>>,

    /// The physical locations where the role can be based. Multiple values
    /// indicate the candidate may choose between them. Recommended format:
    /// "City, Country" or ISO 3166 codes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<String>>,

    /// The language(s) used day-to-day on the team — for meetings, internal
    /// docs, code reviews, Slack, etc. Encoded as BCP 47 tags
    /// (e.g. `["en"]`, `["fr", "en"]`). Distinct from the top-level
    /// `language`, which is the language the posting itself is written in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_languages: Option<Vec<String>>,

    /// Non-compensation perks offered with the role (e.g. health insurance,
    /// meal vouchers, equipment budget, learning stipend).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benefits: Option<Vec<String>>,
}
