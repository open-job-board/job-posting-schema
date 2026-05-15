# JobPosting

A simplified, business-oriented schema for representing job postings. Inspired by [schema.org/JobPosting](https://schema.org/JobPosting), but more concise and tailored as an API interface contract.

A `JobPosting` is a single object composed of the following top-level groups:
- `organization` — who is hiring
- `job` — what the role is
- `contract` — under what terms
- `requirements` — what the candidate needs
- `application` — how to apply
- Top-level metadata: `id`, `language`, `url`, `postedAt`

---

# organization
Type: Object
Description: The organization offering the job.

## name
Type: String
Description: The legal or commonly-used name of the hiring organization.

## overview
Type: String
Description: A short description of the organization's mission, products, and culture. Helps the applicant understand what kind of company is hiring.

## size
Type: Enum (1-10, 11-50, 51-200, 201-500, 501-1000, 1001-5000, 5000+)
Description: The headcount bracket of the organization. Brackets are easier to disclose publicly than exact counts and serve as a quick candidate filter (startup vs scale-up vs enterprise).

## foundedYear
Type: Number
Description: The year the organization was founded (four-digit year, e.g. `2017`). Useful for distinguishing young startups from established companies.

## industries
Type: Industry[]
Description: The industries the organization operates in (see the [Industry vocabulary](#industry-vocabulary)). Multiple values are allowed for diversified employers or conglomerates.

## opportunities
Type: String[]
Description: The career-growth opportunities a successful applicant can expect at this organization (e.g. internal mobility, training budget, mentoring, fast-track to senior roles).

---

# job
Type: Object
Description: The role itself — what the job is about and where it sits in the organization.

## organizationUnit
Type: String
Description: The team, department, or business unit the role belongs to (e.g. "Platform Engineering", "Marketing — Brand").

## industry
Type: Industry
Description: The primary industry this specific role is associated with (see the [Industry vocabulary](#industry-vocabulary)). May differ from the hiring organization's `industries` (e.g. an industrial company hiring for its software division).

## context
Type: String
Description: A short narrative explaining why this role exists right now — a new project, a growth phase, a replacement, etc. Helps the applicant understand the motivation behind the posting.

## title
Type: String
Description: The plain job title as it would appear at the top of the posting (e.g. "Senior Backend Engineer").

## overview
Type: String
Description: A detailed description of the role's purpose and business value, the team size and composition, the working conditions, and what success in the role looks like.

## responsibilities
Type: String[]
Description: The day-to-day duties and ownership areas expected of the role.

## locations
Type: String[]
Description: The physical locations where the role can be based. Multiple values indicate the candidate may choose between them. Recommended format: "City, Country" or ISO 3166 codes.

## workMode
Type: Enum (onsite, hybrid, remote)
Description: The expected work setup. `onsite` requires full physical presence; `hybrid` mixes on-site and remote days (see `contract.telecommuteDaysPerWeek`); `remote` allows fully remote work with no required on-site presence.

## workingLanguages
Type: String[]
Description: The language(s) used day-to-day on the team — for meetings, internal docs, code reviews, Slack, etc. Encoded as BCP 47 tags (e.g. `["en"]`, `["fr", "en"]`). Distinct from the top-level `language`, which is the language the job posting itself is written in.

## benefits
Type: String[]
Description: Non-compensation perks offered with the role (e.g. health insurance, meal vouchers, equipment budget, learning stipend).

---

# contract
Type: Object
Description: The employment terms of the role.

## employmentType
Type: Enum (permanent, fixed-term, freelance, zero-hours, internship)
Description: The legal form of employment. `permanent` is an open-ended contract; `fixed-term` is time-limited employment; `freelance` is an independent-contractor engagement; `zero-hours` is on-call work without a guaranteed minimum; `internship` is a student/trainee position.

## workHours
Type: String
Description: The typical working hours for this job (e.g. "1st shift", "night shift", "9am–6pm CET", "35h/week").

## duration
Type: String
Description: The expected duration of the employment as advertised by the employer. Relevant for postings with a clearly defined period (e.g. seasonal work, maternity-leave replacement, other temporary engagements). Use ISO 8601 duration format when possible (e.g. "P6M").

## startsFrom
Type: Date
Description: The earliest date on which the role can begin.

## immediateStart
Type: Boolean
Description: Whether the employer is looking for someone who can start immediately. When `true`, `startsFrom` is typically the posting date or earlier.

## salary
Type: Object
Description: The compensation offered for the role.

### currency
Type: String
Description: The currency used for the main salary information, encoded using ISO 4217 (e.g. "EUR", "USD").

### period
Type: Enum (hour, day, month, year, project)
Description: The time unit over which `minAmount` and `maxAmount` are computed.

### minAmount
Type: Number
Description: Gross minimum amount for the indicated period in the indicated currency.

### maxAmount
Type: Number
Description: Gross maximum amount for the indicated period in the indicated currency.

### estimated
Type: Boolean
Description: Whether this salary information is estimated (e.g. by a third party or by the agent) rather than explicitly provided by the hiring organization.

## incentives
Type: String[]
Description: Bonus, commission, equity, and other variable compensation aspects of the job.

## telecommuteDaysPerWeek
Type: Number
Description: The number of days per week the role can be performed remotely. Only meaningful when `job.workMode` is `hybrid`.

---

# requirements
Type: Object
Description: What the hiring organization expects from candidates.

## eligibleLocations
Type: String[]
Description: Geographic restrictions on the candidate — typically countries or regions where the applicant must hold the right to work. Distinct from `job.locations`, which is where the role is physically based.

## education
Type: String[]
Description: Required or preferred education credentials (e.g. "Bachelor's in Computer Science", "Engineering degree").

## qualifications
Type: String[]
Description: Required certifications, licenses, or formal qualifications (e.g. "PMP", "AWS Solutions Architect", "Driver's license B").

## administrative
Type: String[]
Description: Legal or administrative prerequisites (e.g. "EU work authorization", "security clearance", "criminal record check").

## yearsOfExperience
Type: Number
Description: The minimum number of years of professional experience required for the role. A scalar field is much easier for the agent to filter on than free-text bullets.

## experiences
Type: String[]
Description: Required prior professional experience expressed as discrete qualitative items (e.g. "Experience in B2B SaaS", "Experience leading a team of 5+", "Worked in a regulated industry"). Quantitative minimums belong in `yearsOfExperience`.

## skills
Type: String[]
Description: Required or preferred hard and soft skills (e.g. "Python", "Kubernetes", "stakeholder management").

## experienceInPlaceOfEducation
Type: Boolean
Description: Whether equivalent professional experience may be accepted in place of the formal education requirement.

---

# application
Type: Object
Description: How to apply for the role.

## email
Type: String
Description: An email address to which applications can be sent.

## contact
Type: String
Description: The name and/or role of a contact person at the hiring organization (e.g. "Jane Doe — Talent Acquisition").

## url
Type: String
Description: A URL to the application form or applicant tracking system.

## deadline
Type: Date
Description: The last date on which applications are accepted. Maps to schema.org's `validThrough` and is useful for the agent to deprioritize or archive expiring postings.

---

# id
Type: String
Description: A stable unique identifier for the job posting, assigned by the system that owns the contract. Used to cross-reference the same posting across API calls and storage layers.

# language
Type: String
Description: The natural language the posting is written in, encoded as a BCP 47 tag (e.g. `"en-US"`, `"fr-FR"`). Lets the agent route posting text to the right NLP pipeline or translation step.

# url
Type: String
Description: The canonical URL of the original job posting.

# postedAt
Type: Date
Description: The date on which the job posting was published.

---

# Industry vocabulary

Shared enum used by `organization.industries` (array) and `job.industry` (single value). Values are stable kebab-case slugs; labels are intended for display and may be re-worded without breaking the contract.

| Value | Label |
|---|---|
| `advertising-marketing-agency` | Advertising / Marketing / Agency |
| `architecture` | Architecture |
| `banking-insurance-finance` | Banking / Insurance / Finance |
| `consulting-audit` | Consulting / Audit |
| `corporate-services` | Corporate Services |
| `culture-media-entertainment` | Culture / Media / Entertainment |
| `distribution` | Distribution |
| `education-training-recruitment` | Education / Training / Recruitment |
| `engineering` | Engineering |
| `fashion-luxury-beauty-lifestyle` | Fashion / Luxury / Beauty / Lifestyle |
| `food-and-beverage` | Food and Beverage |
| `health-social-environment` | Health / Social / Environment |
| `hotel-tourism-leisure` | Hotel / Tourism / Leisure |
| `industry` | Industry |
| `legal-law` | Legal / Law |
| `mobility-transport` | Mobility / Transport |
| `nonprofit-association` | Nonprofit / Association |
| `public-administration` | Public Administration |
| `real-estate` | Real Estate |
| `tech` | Tech |
