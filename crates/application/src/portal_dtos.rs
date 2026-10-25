//! # Enterprise Portal Data Transfer Objects (DTOs)
//!
//! Strongly-typed DTOs for Client Portal and Admin Dashboard operations,
//! serializable for WASM client hydration and SSR server functions.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ─── CLIENT OVERVIEW DTO ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientOverviewDto {
    pub client_name: String,
    pub organization_name: String,
    pub active_services_count: i64,
    pub active_projects_count: i64,
    pub pending_requests_count: i64,
    pub unread_notifications_count: i64,
    pub recent_services: Vec<ClientServiceItemDto>,
    pub recent_projects: Vec<ProjectItemDto>,
    pub recent_notifications: Vec<NotificationItemDto>,
}

// ─── SERVICES & CATALOG DTOS ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClientServiceItemDto {
    pub id: String,
    pub service_id: String,
    pub title: String,
    pub title_ar: String,
    pub category: String,
    pub package_name: String,
    pub status: String,
    pub price: String,
    pub currency: String,
    pub sla_tier: String,
    pub assigned_lead: String,
    pub start_date: String,
    pub renewal_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CatalogPackageDto {
    pub id: String,
    pub name: String,
    pub name_ar: String,
    pub price: String,
    pub currency: String,
    pub billing_period: String,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CatalogServiceDto {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub title_ar: String,
    pub category: String,
    pub description: String,
    pub description_ar: String,
    pub icon: String,
    pub packages: Vec<CatalogPackageDto>,
}

// ─── PROJECTS & MILESTONES DTOS ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MilestoneDto {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProjectItemDto {
    pub id: String,
    pub client_service_id: Option<String>,
    pub title: String,
    pub description: String,
    pub status: String,
    pub progress_pct: i32,
    pub priority: String,
    pub deadline: Option<String>,
    pub assigned_team: String,
    pub milestones: Vec<MilestoneDto>,
}

// ─── CHANGE REQUESTS DTOS ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChangeRequestItemDto {
    pub id: String,
    pub project_id: Option<String>,
    pub requested_by_name: String,
    pub title: String,
    pub scope_description: String,
    pub estimated_cost: String,
    pub currency: String,
    pub estimated_days: i32,
    pub status: String,
    pub review_notes: Option<String>,
    pub created_at: String,
}

// ─── CONVERSATIONS & MESSAGING DTOS ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MessageItemDto {
    pub id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_role: String,
    pub body: String,
    pub is_internal: bool,
    pub sent_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversationItemDto {
    pub id: String,
    pub title: String,
    pub channel_type: String,
    pub related_project_id: Option<String>,
    pub updated_at: String,
    pub messages: Vec<MessageItemDto>,
}

// ─── DELIVERABLES & FILES DTOS ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeliverableItemDto {
    pub id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub file_name: String,
    pub file_url: String,
    pub file_size_formatted: String,
    pub version: String,
    pub status: String,
    pub published_at: String,
}

// ─── INVOICES & BILLING DTOS ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InvoiceLineItemDto {
    pub id: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price: String,
    pub total_amount: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InvoiceItemDto {
    pub id: String,
    pub invoice_number: String,
    pub status: String,
    pub subtotal: String,
    pub tax_amount: String,
    pub total_amount: String,
    pub currency: String,
    pub due_date: String,
    pub issued_at: String,
    pub paid_at: Option<String>,
    pub payment_method: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<InvoiceLineItemDto>,
}

// ─── NOTIFICATIONS & AUDIT DTOS ──────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NotificationItemDto {
    pub id: String,
    pub title: String,
    pub title_ar: String,
    pub body: String,
    pub body_ar: String,
    pub category: String,
    pub link_url: Option<String>,
    pub is_read: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuditLogItemDto {
    pub id: String,
    pub actor_name: String,
    pub actor_role: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: String,
    pub details: Option<String>,
    pub ip_address: String,
    pub recorded_at: String,
}

// ─── ADMIN DASHBOARD DTOS ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AdminClientSummaryDto {
    pub tenant_id: String,
    pub name: String,
    pub slug: String,
    pub created_at: String,
    pub active_services_count: i64,
    pub active_projects_count: i64,
    pub total_billed_sar: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AdminOverviewDto {
    pub total_clients: i64,
    pub active_services: i64,
    pub active_projects: i64,
    pub pending_requests: i64,
    pub total_revenue_sar: String,
    pub clients: Vec<AdminClientSummaryDto>,
    pub recent_activity: Vec<AuditLogItemDto>,
}
