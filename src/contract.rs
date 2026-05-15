use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Legal form of employment. `permanent` is an open-ended contract;
/// `fixed-term` is time-limited employment; `freelance` is an
/// independent-contractor engagement; `zero-hours` is on-call work without
/// a guaranteed minimum; `internship` is a student/trainee position.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum EmploymentType {
    Permanent,
    FixedTerm,
    Freelance,
    ZeroHours,
    Internship,
}

/// Time unit over which `minAmount` and `maxAmount` are computed.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SalaryPeriod {
    Hour,
    Day,
    Month,
    Year,
    Project,
}

/// Compensation offered for the role.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Salary {
    /// Currency used for the main salary information, encoded using ISO 4217
    /// (e.g. "EUR", "USD").
    pub currency: String,

    /// Time unit over which `minAmount` and `maxAmount` are computed.
    pub period: SalaryPeriod,

    /// Gross minimum amount for the indicated period in the indicated currency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_amount: Option<f64>,

    /// Gross maximum amount for the indicated period in the indicated currency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,

    /// Whether this salary information is estimated (e.g. by a third party
    /// or by the agent) rather than explicitly provided by the hiring
    /// organization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated: Option<bool>,
}

/// The employment terms of the role.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// Legal form of employment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<EmploymentType>,

    /// Typical working hours for this job (e.g. "1st shift", "night shift",
    /// "9am–6pm CET", "35h/week").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<String>,

    /// Expected duration of the employment as advertised by the employer.
    /// Relevant for postings with a clearly defined period (e.g. seasonal
    /// work, maternity-leave replacement, other temporary engagements).
    /// Use ISO 8601 duration format when possible (e.g. "P6M").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    /// Earliest date on which the role can begin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starts_from: Option<NaiveDate>,

    /// Whether the employer is looking for someone who can start immediately.
    /// When `true`, `startsFrom` is typically the posting date or earlier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub immediate_start: Option<bool>,

    /// Compensation offered for the role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub salary: Option<Salary>,

    /// Bonus, commission, equity, and other variable compensation aspects
    /// of the job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incentives: Option<Vec<String>>,

    /// Number of days per week the role can be performed remotely. Only
    /// meaningful when `job.workMode` is `hybrid`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telecommute_days_per_week: Option<u8>,
}
