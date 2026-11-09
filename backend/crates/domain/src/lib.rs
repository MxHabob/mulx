use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrganizationId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BranchId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurrencyCode(pub String);

impl CurrencyCode {
    pub fn usd() -> Self {
        Self("USD".to_string())
    }

    pub fn eur() -> Self {
        Self("EUR".to_string())
    }

    pub fn bhd() -> Self {
        Self("BHD".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: CurrencyCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleKind {
    Core,
    Fin,
    Sal,
    Pur,
    Inv,
    Pos,
    Crm,
    Scm,
    Mfg,
    Hr,
    Ast,
    Prj,
    Dms,
    Bi,
    Int,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleBoundary {
    pub kind: ModuleKind,
    pub name: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub legal_name: String,
    pub default_currency: CurrencyCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub organization_id: OrganizationId,
    pub email: String,
    pub display_name: String,
}
