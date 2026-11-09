use mulx_domain::{Money, Organization, OrganizationId, User, UserId};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("organization not found: {0:?}")]
    OrganizationNotFound(OrganizationId),
    #[error("user not found: {0:?}")]
    UserNotFound(UserId),
    #[error("validation failed: {0}")]
    Validation(String),
}

#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub ready: bool,
    pub version: &'static str,
    pub service: &'static str,
}

#[derive(Debug, Clone)]
pub struct FinancialSummary {
    pub organization: Organization,
    pub total_accounts: u64,
    pub ledger_total: Money,
}

pub struct CoreDomainService;

impl CoreDomainService {
    pub fn health_check() -> HealthStatus {
        HealthStatus {
            ready: true,
            version: env!("CARGO_PKG_VERSION"),
            service: "mulx-api",
        }
    }

    pub fn create_organization(name: &str, legal_name: &str) -> Result<Organization, AppError> {
        if name.trim().is_empty() || legal_name.trim().is_empty() {
            return Err(AppError::Validation(
                "organization name and legal name are required".to_string(),
            ));
        }

        Ok(Organization {
            id: OrganizationId(uuid::Uuid::new_v4()),
            name: name.to_string(),
            legal_name: legal_name.to_string(),
            default_currency: mulx_domain::CurrencyCode::usd(),
        })
    }

    pub fn create_user(
        organization_id: OrganizationId,
        email: &str,
        display_name: &str,
    ) -> Result<User, AppError> {
        if !email.contains('@') {
            return Err(AppError::Validation("valid email is required".to_string()));
        }

        Ok(User {
            id: UserId(uuid::Uuid::new_v4()),
            organization_id,
            email: email.to_string(),
            display_name: display_name.to_string(),
        })
    }
}
