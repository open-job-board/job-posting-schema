use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Shared industry vocabulary. Values are stable kebab-case slugs; labels
/// are for display only and may be re-worded without breaking the contract.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Industry {
    AdvertisingMarketingAgency,
    Architecture,
    BankingInsuranceFinance,
    ConsultingAudit,
    CorporateServices,
    CultureMediaEntertainment,
    Distribution,
    EducationTrainingRecruitment,
    Engineering,
    FashionLuxuryBeautyLifestyle,
    FoodAndBeverage,
    HealthSocialEnvironment,
    HotelTourismLeisure,
    Industry,
    LegalLaw,
    MobilityTransport,
    NonprofitAssociation,
    PublicAdministration,
    RealEstate,
    Tech,
}
