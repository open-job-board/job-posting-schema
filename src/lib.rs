//! Type definitions for the JobPosting API contract.
//!
//! The [`JobPosting`] struct mirrors the schema described in `README.md`
//! and derives `serde` traits for (de)serialization and `schemars::JsonSchema`
//! for JSON Schema generation. The `generate-schema` binary writes the
//! generated schema to stdout.

pub mod application;
pub mod contract;
pub mod hiring_organization;
pub mod industry;
pub mod job;
pub mod requirements;

pub use application::Application;
pub use contract::{Contract, EmploymentType, Salary, SalaryPeriod};
pub use hiring_organization::{Organization, OrganizationSize};
pub use industry::Industry;
pub use job::{Geolocation, Job, WorkMode};
pub use requirements::Requirements;

use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A simplified, business-oriented job posting.
///
/// Composed of five top-level groups (`organization`, `job`,
/// `contract`, `requirements`, `application`) plus posting-level metadata
/// (`id`, `language`, `url`, `postedAt`).
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobPosting {
    /// Stable unique identifier for the job posting, assigned by the system
    /// that owns the contract. Used to cross-reference the same posting
    /// across API calls and storage layers.
    pub id: String,

    /// Natural language the posting is written in, encoded as a BCP 47 tag
    /// (e.g. `en-US`, `fr-FR`). Lets the agent route posting text to the
    /// right NLP pipeline or translation step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Date on which the job posting was published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<NaiveDate>,

    /// Canonical URL of the original job posting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The organization offering the job.
    pub organization: Organization,

    /// The role itself — what the job is about and where it sits in the
    /// organization.
    pub job: Job,

    /// The employment terms of the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,

    /// What the hiring organization expects from candidates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Requirements>,

    /// How to apply for the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
}
