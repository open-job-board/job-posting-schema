use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::industry::Industry;

/// Headcount bracket of the organization. Brackets are easier to disclose
/// publicly than exact counts and serve as a quick candidate filter
/// (startup vs scale-up vs enterprise).
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganizationSize {
    #[serde(rename = "1-10")]
    Range1To10,
    #[serde(rename = "11-50")]
    Range11To50,
    #[serde(rename = "51-200")]
    Range51To200,
    #[serde(rename = "201-500")]
    Range201To500,
    #[serde(rename = "501-1000")]
    Range501To1000,
    #[serde(rename = "1001-5000")]
    Range1001To5000,
    #[serde(rename = "5000+")]
    Range5000Plus,
}

/// The organization offering the job.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// The legal or commonly-used name of the hiring organization.
    pub name: String,

    /// A short description of the organization's mission, products, and
    /// culture. Helps the applicant understand what kind of company is hiring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,

    /// The headcount bracket of the organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<OrganizationSize>,

    /// The year the organization was founded (four-digit year, e.g. `2017`).
    /// Useful for distinguishing young startups from established companies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub founded_year: Option<u16>,

    /// The industries the organization operates in. Multiple values are
    /// allowed for diversified employers or conglomerates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industries: Option<Vec<Industry>>,

    /// The career-growth opportunities a successful applicant can expect
    /// at this organization (e.g. internal mobility, training budget,
    /// mentoring, fast-track to senior roles).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opportunities: Option<Vec<String>>,
}
