use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// What the hiring organization expects from candidates.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
    /// Geographic restrictions on the candidate — typically countries or
    /// regions where the applicant must hold the right to work. Distinct
    /// from `job.locations`, which is where the role is physically based.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligible_locations: Option<Vec<String>>,

    /// Required or preferred education credentials (e.g. "Bachelor's in
    /// Computer Science", "Engineering degree").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub education: Option<Vec<String>>,

    /// Required certifications, licenses, or formal qualifications
    /// (e.g. "PMP", "AWS Solutions Architect", "Driver's license B").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<Vec<String>>,

    /// Legal or administrative prerequisites (e.g. "EU work authorization",
    /// "security clearance", "criminal record check").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative: Option<Vec<String>>,

    /// Minimum number of years of professional experience required for the
    /// role. A scalar field is much easier to filter on than free-text bullets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub years_of_experience: Option<u8>,

    /// Required prior professional experience expressed as discrete
    /// qualitative items (e.g. "Experience in B2B SaaS", "Experience
    /// leading a team of 5+", "Worked in a regulated industry").
    /// Quantitative minimums belong in `yearsOfExperience`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiences: Option<Vec<String>>,

    /// Required or preferred hard and soft skills (e.g. "Python",
    /// "Kubernetes", "stakeholder management").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<String>>,

    /// Whether equivalent professional experience may be accepted in place
    /// of the formal education requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience_in_place_of_education: Option<bool>,
}
