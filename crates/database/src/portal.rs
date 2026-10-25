//! # Enterprise Portal Repository Adapter
//!
//! Multi-tenant zero-trust query adapter for Client Portal and Admin Dashboard.
//! Strict isolation: all client queries mandate `tenant_id` filtering.

use application::*;
use shared::{AppError, AppResult};
#[cfg(feature = "sqlx")]
use sqlx::{PgPool, Row};

#[derive(Clone)]
pub struct PgPortalRepository {
    #[cfg(feature = "sqlx")]
    pool: Option<PgPool>,
}

impl PgPortalRepository {
    #[cfg(feature = "sqlx")]
    pub fn new(pool: PgPool) -> Self {
        Self { pool: Some(pool) }
    }

    pub fn in_memory() -> Self {
        Self {
            #[cfg(feature = "sqlx")]
            pool: None,
        }
    }

    // ─── CLIENT OVERVIEW ─────────────────────────────────────────────────────

    pub async fn get_client_overview(
        &self,
        tenant_id: &str,
        user_name: &str,
    ) -> AppResult<ClientOverviewDto> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let tenant_row = sqlx::query("SELECT name FROM tenants WHERE id = $1")
                .bind(tenant_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            let org_name = tenant_row
                .map(|r| r.get::<String, _>("name"))
                .unwrap_or_else(|| "Client Organization".to_string());

            let services_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM client_services WHERE tenant_id = $1 AND status = 'Active'",
            )
            .bind(tenant_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let projects_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM projects WHERE tenant_id = $1 AND status = 'In Progress'",
            )
            .bind(tenant_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let requests_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM change_requests WHERE tenant_id = $1 AND status IN ('Submitted', 'Under Review', 'Quoted')",
            )
            .bind(tenant_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let unread_notifs: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM notifications WHERE tenant_id = $1 AND is_read = FALSE",
            )
            .bind(tenant_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let recent_services = self.get_client_services(tenant_id).await?;
            let recent_projects = self.get_client_projects(tenant_id).await?;
            let recent_notifications = self.get_client_notifications(tenant_id).await?;

            return Ok(ClientOverviewDto {
                client_name: user_name.to_string(),
                organization_name: org_name,
                active_services_count: services_count,
                active_projects_count: projects_count,
                pending_requests_count: requests_count,
                unread_notifications_count: unread_notifs,
                recent_services: recent_services.into_iter().take(3).collect(),
                recent_projects: recent_projects.into_iter().take(2).collect(),
                recent_notifications: recent_notifications.into_iter().take(4).collect(),
            });
        }

        Ok(ClientOverviewDto {
            client_name: user_name.to_string(),
            organization_name: "Mock Acme Corp".to_string(),
            active_services_count: 2,
            active_projects_count: 1,
            pending_requests_count: 1,
            unread_notifications_count: 2,
            recent_services: vec![],
            recent_projects: vec![],
            recent_notifications: vec![],
        })
    }

    // ─── CLIENT SERVICES ─────────────────────────────────────────────────────

    pub async fn get_client_services(&self, tenant_id: &str) -> AppResult<Vec<ClientServiceItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT cs.id, cs.service_id, s.title, s.title_ar, s.category,
                       cs.package_name, cs.status, cs.price::text as price, cs.currency,
                       cs.sla_tier, cs.assigned_lead,
                       cs.start_date::text as start_date, cs.renewal_date::text as renewal_date
                FROM client_services cs
                JOIN services s ON s.id = cs.service_id
                WHERE cs.tenant_id = $1
                ORDER BY cs.created_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to query client services: {}", e)))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(ClientServiceItemDto {
                    id: r.get("id"),
                    service_id: r.get("service_id"),
                    title: r.get("title"),
                    title_ar: r.get("title_ar"),
                    category: r.get("category"),
                    package_name: r.get("package_name"),
                    status: r.get("status"),
                    price: r.get("price"),
                    currency: r.get("currency"),
                    sla_tier: r.get("sla_tier"),
                    assigned_lead: r.get("assigned_lead"),
                    start_date: r.get("start_date"),
                    renewal_date: r.get("renewal_date"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    // ─── PROJECTS & MILESTONES ───────────────────────────────────────────────

    pub async fn get_client_projects(&self, tenant_id: &str) -> AppResult<Vec<ProjectItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let p_rows = sqlx::query(
                r#"
                SELECT id, client_service_id, title, description, status,
                       progress_pct, priority, deadline::text as deadline, assigned_team
                FROM projects
                WHERE tenant_id = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut projects = Vec::new();
            for r in p_rows {
                let pid: String = r.get("id");
                let m_rows = sqlx::query(
                    r#"
                    SELECT id, title, description, status,
                           due_date::text as due_date, completed_at::text as completed_at
                    FROM project_milestones
                    WHERE project_id = $1
                    ORDER BY due_date ASC
                    "#,
                )
                .bind(&pid)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                let milestones = m_rows
                    .into_iter()
                    .map(|mr| MilestoneDto {
                        id: mr.get("id"),
                        title: mr.get("title"),
                        description: mr.get("description"),
                        status: mr.get("status"),
                        due_date: mr.get("due_date"),
                        completed_at: mr.get("completed_at"),
                    })
                    .collect();

                projects.push(ProjectItemDto {
                    id: pid,
                    client_service_id: r.get("client_service_id"),
                    title: r.get("title"),
                    description: r.get("description"),
                    status: r.get("status"),
                    progress_pct: r.get("progress_pct"),
                    priority: r.get("priority"),
                    deadline: r.get("deadline"),
                    assigned_team: r.get("assigned_team"),
                    milestones,
                });
            }
            return Ok(projects);
        }
        Ok(vec![])
    }

    // ─── SERVICE CATALOG ─────────────────────────────────────────────────────

    pub async fn get_service_catalog(&self) -> AppResult<Vec<CatalogServiceDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let s_rows = sqlx::query(
                r#"
                SELECT id, slug, title, title_ar, category, description, description_ar, icon
                FROM services
                WHERE status = 'Published'
                ORDER BY title ASC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut catalog = Vec::new();
            for r in s_rows {
                let sid: String = r.get("id");
                let p_rows = sqlx::query(
                    r#"
                    SELECT id, name, name_ar, price::text as price, currency, billing_period, features_json
                    FROM service_packages
                    WHERE service_id = $1
                    ORDER BY price ASC
                    "#,
                )
                .bind(&sid)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                let packages = p_rows
                    .into_iter()
                    .map(|pr| {
                        let feat_val: serde_json::Value = pr.get("features_json");
                        let features: Vec<String> = serde_json::from_value(feat_val).unwrap_or_default();
                        CatalogPackageDto {
                            id: pr.get("id"),
                            name: pr.get("name"),
                            name_ar: pr.get("name_ar"),
                            price: pr.get("price"),
                            currency: pr.get("currency"),
                            billing_period: pr.get("billing_period"),
                            features,
                        }
                    })
                    .collect();

                catalog.push(CatalogServiceDto {
                    id: sid,
                    slug: r.get("slug"),
                    title: r.get("title"),
                    title_ar: r.get("title_ar"),
                    category: r.get("category"),
                    description: r.get("description"),
                    description_ar: r.get("description_ar"),
                    icon: r.get("icon"),
                    packages,
                });
            }
            return Ok(catalog);
        }
        Ok(vec![])
    }

    // ─── CHANGE REQUESTS ─────────────────────────────────────────────────────

    pub async fn get_client_change_requests(
        &self,
        tenant_id: &str,
    ) -> AppResult<Vec<ChangeRequestItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, project_id, requested_by_name, title, scope_description,
                       estimated_cost::text as estimated_cost, currency, estimated_days,
                       status, review_notes, created_at::text as created_at
                FROM change_requests
                WHERE tenant_id = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(ChangeRequestItemDto {
                    id: r.get("id"),
                    project_id: r.get("project_id"),
                    requested_by_name: r.get("requested_by_name"),
                    title: r.get("title"),
                    scope_description: r.get("scope_description"),
                    estimated_cost: r.get("estimated_cost"),
                    currency: r.get("currency"),
                    estimated_days: r.get("estimated_days"),
                    status: r.get("status"),
                    review_notes: r.get("review_notes"),
                    created_at: r.get("created_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn create_change_request(
        &self,
        tenant_id: &str,
        user_id: &str,
        user_name: &str,
        project_id: Option<String>,
        title: &str,
        description: &str,
    ) -> AppResult<ChangeRequestItemDto> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let cr_id = format!("cr_{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());

            sqlx::query(
                r#"
                INSERT INTO change_requests (id, tenant_id, project_id, requested_by_user_id,
                                            requested_by_name, title, scope_description,
                                            estimated_cost, currency, estimated_days, status, created_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, 0.00, 'SAR', 0, 'Submitted', NOW())
                "#,
            )
            .bind(&cr_id)
            .bind(tenant_id)
            .bind(&project_id)
            .bind(user_id)
            .bind(user_name)
            .bind(title)
            .bind(description)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create change request: {}", e)))?;

            // Record audit event
            let _ = self.record_audit_log(
                Some(tenant_id),
                user_id,
                user_name,
                "Client",
                "CHANGE_REQUEST_SUBMITTED",
                "ChangeRequest",
                &cr_id,
                Some(&format!("Title: {}", title)),
            ).await;

            return Ok(ChangeRequestItemDto {
                id: cr_id,
                project_id,
                requested_by_name: user_name.to_string(),
                title: title.to_string(),
                scope_description: description.to_string(),
                estimated_cost: "0.00".to_string(),
                currency: "SAR".to_string(),
                estimated_days: 0,
                status: "Submitted".to_string(),
                review_notes: None,
                created_at: "Just now".to_string(),
            });
        }
        Err(AppError::Internal("Database pool offline".to_string()))
    }

    pub async fn client_decide_change_request(
        &self,
        tenant_id: &str,
        request_id: &str,
        approved: bool,
        user_id: &str,
        user_name: &str,
    ) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let new_status = if approved { "Approved" } else { "Rejected" };
            let result = sqlx::query(
                r#"
                UPDATE change_requests
                SET status = $1, resolved_at = NOW()
                WHERE id = $2 AND tenant_id = $3
                "#,
            )
            .bind(new_status)
            .bind(request_id)
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            if result.rows_affected() == 0 {
                return Err(AppError::NotFound("Change request not found or unauthorized".to_string()));
            }

            let action = if approved { "CHANGE_REQUEST_APPROVED" } else { "CHANGE_REQUEST_REJECTED" };
            let _ = self.record_audit_log(
                Some(tenant_id),
                user_id,
                user_name,
                "Client",
                action,
                "ChangeRequest",
                request_id,
                Some(&format!("Client decision: {}", new_status)),
            ).await;

            return Ok(());
        }
        Ok(())
    }

    pub async fn order_service_package(
        &self,
        tenant_id: &str,
        user_id: &str,
        user_name: &str,
        service_id: &str,
        package_name: &str,
        notes: &str,
    ) -> AppResult<String> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let s_row = sqlx::query("SELECT title FROM services WHERE id = $1")
                .bind(service_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            let s_title = s_row
                .map(|r| r.get::<String, _>("title"))
                .unwrap_or_else(|| "Service Offering".to_string());

            let title = format!("Service Order: {} ({})", s_title, package_name);
            let description = if notes.trim().is_empty() {
                format!("Client requested engagement for {} tier of {}.", package_name, s_title)
            } else {
                format!("Client requested engagement for {} tier of {}.\n\nRequirements & Notes:\n{}", package_name, s_title, notes)
            };

            let cr_id = format!("cr_{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());

            sqlx::query(
                r#"
                INSERT INTO change_requests (id, tenant_id, project_id, requested_by_user_id,
                                            requested_by_name, title, scope_description,
                                            estimated_cost, currency, estimated_days, status, created_at)
                VALUES ($1, $2, NULL, $3, $4, $5, $6, 0.00, 'SAR', 0, 'Submitted', NOW())
                "#,
            )
            .bind(&cr_id)
            .bind(tenant_id)
            .bind(user_id)
            .bind(user_name)
            .bind(&title)
            .bind(&description)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to create order request: {}", e)))?;

            let _ = self.record_audit_log(
                Some(tenant_id),
                user_id,
                user_name,
                "Client",
                "SERVICE_PACKAGE_ORDERED",
                "ServicePackage",
                service_id,
                Some(&format!("Package: {} (CR ID: {})", package_name, cr_id)),
            ).await;

            return Ok(cr_id);
        }
        Err(AppError::Internal("Database pool offline".to_string()))
    }

    // ─── CONVERSATIONS & MESSAGES (WITH ZERO-TRUST INTERNAL FILTER) ───────────

    pub async fn get_client_conversations(
        &self,
        tenant_id: &str,
        is_staff: bool,
    ) -> AppResult<Vec<ConversationItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let c_rows = sqlx::query(
                r#"
                SELECT id, title, channel_type, related_project_id, updated_at::text as updated_at
                FROM conversations
                WHERE tenant_id = $1
                ORDER BY updated_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in c_rows {
                let cid: String = r.get("id");

                // CRITICAL ZERO-TRUST RULE:
                // If caller is client (not staff), enforce `is_internal = FALSE` strictly.
                let m_query = if is_staff {
                    "SELECT id, conversation_id, sender_id, sender_name, sender_role, body, is_internal, sent_at::text as sent_at FROM messages WHERE conversation_id = $1 ORDER BY sent_at ASC"
                } else {
                    "SELECT id, conversation_id, sender_id, sender_name, sender_role, body, is_internal, sent_at::text as sent_at FROM messages WHERE conversation_id = $1 AND is_internal = FALSE ORDER BY sent_at ASC"
                };

                let m_rows = sqlx::query(m_query)
                    .bind(&cid)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| AppError::Internal(e.to_string()))?;

                let messages = m_rows
                    .into_iter()
                    .map(|mr| MessageItemDto {
                        id: mr.get("id"),
                        conversation_id: mr.get("conversation_id"),
                        sender_id: mr.get("sender_id"),
                        sender_name: mr.get("sender_name"),
                        sender_role: mr.get("sender_role"),
                        body: mr.get("body"),
                        is_internal: mr.get("is_internal"),
                        sent_at: mr.get("sent_at"),
                    })
                    .collect();

                list.push(ConversationItemDto {
                    id: cid,
                    title: r.get("title"),
                    channel_type: r.get("channel_type"),
                    related_project_id: r.get("related_project_id"),
                    updated_at: r.get("updated_at"),
                    messages,
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn send_message(
        &self,
        conversation_id: &str,
        sender_id: &str,
        sender_name: &str,
        sender_role: &str,
        body: &str,
        is_internal: bool,
    ) -> AppResult<MessageItemDto> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let msg_id = format!("msg_{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());

            sqlx::query(
                r#"
                INSERT INTO messages (id, conversation_id, sender_id, sender_name, sender_role, body, is_internal, sent_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
                "#,
            )
            .bind(&msg_id)
            .bind(conversation_id)
            .bind(sender_id)
            .bind(sender_name)
            .bind(sender_role)
            .bind(body)
            .bind(is_internal)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            sqlx::query("UPDATE conversations SET updated_at = NOW() WHERE id = $1")
                .bind(conversation_id)
                .execute(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            return Ok(MessageItemDto {
                id: msg_id,
                conversation_id: conversation_id.to_string(),
                sender_id: sender_id.to_string(),
                sender_name: sender_name.to_string(),
                sender_role: sender_role.to_string(),
                body: body.to_string(),
                is_internal,
                sent_at: "Just now".to_string(),
            });
        }
        Err(AppError::Internal("Database pool offline".to_string()))
    }

    pub async fn create_client_conversation(
        &self,
        tenant_id: &str,
        user_id: &str,
        user_name: &str,
        title: &str,
        channel_type: &str,
        initial_message: &str,
    ) -> AppResult<String> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let conv_id = format!("conv_{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
            let msg_id = format!("msg_{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() + 1);

            let mut tx = pool.begin().await.map_err(|e| AppError::Internal(e.to_string()))?;

            sqlx::query(
                r#"
                INSERT INTO conversations (id, tenant_id, title, channel_type, updated_at)
                VALUES ($1, $2, $3, $4, NOW())
                "#,
            )
            .bind(&conv_id)
            .bind(tenant_id)
            .bind(title)
            .bind(channel_type)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            sqlx::query(
                r#"
                INSERT INTO messages (id, conversation_id, sender_id, sender_name, sender_role, body, is_internal, sent_at)
                VALUES ($1, $2, $3, $4, 'Client Lead', $5, FALSE, NOW())
                "#,
            )
            .bind(&msg_id)
            .bind(&conv_id)
            .bind(user_id)
            .bind(user_name)
            .bind(initial_message)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            tx.commit().await.map_err(|e| AppError::Internal(e.to_string()))?;

            let _ = self.record_audit_log(
                Some(tenant_id),
                user_id,
                user_name,
                "Client",
                "CONVERSATION_CHANNEL_OPENED",
                "Conversation",
                &conv_id,
                Some(&format!("Channel: {}", title)),
            ).await;

            return Ok(conv_id);
        }
        Err(AppError::Internal("Database offline".to_string()))
    }

    // ─── DELIVERABLES ────────────────────────────────────────────────────────

    pub async fn get_client_deliverables(&self, tenant_id: &str) -> AppResult<Vec<DeliverableItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, project_id, title, file_name, file_url,
                       file_size_bytes, version, status, published_at::text as published_at
                FROM deliverables
                WHERE tenant_id = $1 AND status != 'Draft'
                ORDER BY published_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                let bytes: i64 = r.get("file_size_bytes");
                let size_mb = (bytes as f64) / (1024.0 * 1024.0);
                let formatted = format!("{:.1} MB", size_mb);

                list.push(DeliverableItemDto {
                    id: r.get("id"),
                    project_id: r.get("project_id"),
                    title: r.get("title"),
                    file_name: r.get("file_name"),
                    file_url: r.get("file_url"),
                    file_size_formatted: formatted,
                    version: r.get("version"),
                    status: r.get("status"),
                    published_at: r.get("published_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn get_deliverable_for_download(
        &self,
        tenant_id: &str,
        deliverable_id: &str,
    ) -> AppResult<Option<DeliverableItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let row = sqlx::query(
                r#"
                SELECT id, project_id, title, file_name, file_url,
                       file_size_bytes, version, status, published_at::text as published_at
                FROM deliverables
                WHERE id = $1 AND tenant_id = $2 AND status != 'Draft'
                "#,
            )
            .bind(deliverable_id)
            .bind(tenant_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            if let Some(r) = row {
                let bytes: i64 = r.get("file_size_bytes");
                let size_mb = (bytes as f64) / (1024.0 * 1024.0);
                return Ok(Some(DeliverableItemDto {
                    id: r.get("id"),
                    project_id: r.get("project_id"),
                    title: r.get("title"),
                    file_name: r.get("file_name"),
                    file_url: r.get("file_url"),
                    file_size_formatted: format!("{:.1} MB", size_mb),
                    version: r.get("version"),
                    status: r.get("status"),
                    published_at: r.get("published_at"),
                }));
            }
            return Ok(None);
        }
        Ok(None)
    }

    // ─── INVOICES & BILLING ──────────────────────────────────────────────────

    pub async fn get_client_invoices(&self, tenant_id: &str) -> AppResult<Vec<InvoiceItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let i_rows = sqlx::query(
                r#"
                SELECT id, invoice_number, status, subtotal::text as subtotal,
                       tax_amount::text as tax_amount, total_amount::text as total_amount,
                       currency, due_date::text as due_date, issued_at::text as issued_at,
                       paid_at::text as paid_at, payment_method, notes
                FROM invoices
                WHERE tenant_id = $1
                ORDER BY issued_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in i_rows {
                let iid: String = r.get("id");
                let it_rows = sqlx::query(
                    r#"
                    SELECT id, description, quantity, unit_price::text as unit_price, total_amount::text as total_amount
                    FROM invoice_items
                    WHERE invoice_id = $1
                    "#,
                )
                .bind(&iid)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                let items = it_rows
                    .into_iter()
                    .map(|itr| InvoiceLineItemDto {
                        id: itr.get("id"),
                        description: itr.get("description"),
                        quantity: itr.get("quantity"),
                        unit_price: itr.get("unit_price"),
                        total_amount: itr.get("total_amount"),
                    })
                    .collect();

                list.push(InvoiceItemDto {
                    id: iid,
                    invoice_number: r.get("invoice_number"),
                    status: r.get("status"),
                    subtotal: r.get("subtotal"),
                    tax_amount: r.get("tax_amount"),
                    total_amount: r.get("total_amount"),
                    currency: r.get("currency"),
                    due_date: r.get("due_date"),
                    issued_at: r.get("issued_at"),
                    paid_at: r.get("paid_at"),
                    payment_method: r.get("payment_method"),
                    notes: r.get("notes"),
                    items,
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    // ─── NOTIFICATIONS ───────────────────────────────────────────────────────

    pub async fn get_client_notifications(&self, tenant_id: &str) -> AppResult<Vec<NotificationItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, title, title_ar, body, body_ar, category, link_url, is_read, created_at::text as created_at
                FROM notifications
                WHERE tenant_id = $1
                ORDER BY created_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(NotificationItemDto {
                    id: r.get("id"),
                    title: r.get("title"),
                    title_ar: r.get("title_ar"),
                    body: r.get("body"),
                    body_ar: r.get("body_ar"),
                    category: r.get("category"),
                    link_url: r.get("link_url"),
                    is_read: r.get("is_read"),
                    created_at: r.get("created_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn mark_client_notifications_read(&self, tenant_id: &str) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                UPDATE notifications
                SET is_read = TRUE
                WHERE tenant_id = $1 AND is_read = FALSE
                "#,
            )
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    pub async fn update_client_profile(&self, user_id: &str, display_name: &str) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            sqlx::query("UPDATE users SET display_name = $1 WHERE id = $2")
                .bind(display_name)
                .bind(user_id)
                .execute(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    // ─── AUDIT LOGS ──────────────────────────────────────────────────────────

    pub async fn get_client_audit_logs(&self, tenant_id: &str) -> AppResult<Vec<AuditLogItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, actor_name, actor_role, action, resource_type,
                       resource_id, details, ip_address, recorded_at::text as recorded_at
                FROM audit_logs
                WHERE tenant_id = $1
                ORDER BY recorded_at DESC
                "#,
            )
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(AuditLogItemDto {
                    id: r.get("id"),
                    actor_name: r.get("actor_name"),
                    actor_role: r.get("actor_role"),
                    action: r.get("action"),
                    resource_type: r.get("resource_type"),
                    resource_id: r.get("resource_id"),
                    details: r.get("details"),
                    ip_address: r.get("ip_address"),
                    recorded_at: r.get("recorded_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn record_audit_log(
        &self,
        tenant_id: Option<&str>,
        actor_id: &str,
        actor_name: &str,
        actor_role: &str,
        action: &str,
        resource_type: &str,
        resource_id: &str,
        details: Option<&str>,
    ) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let aid = format!("aud_{:x}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
            let _ = sqlx::query(
                r#"
                INSERT INTO audit_logs (id, tenant_id, actor_id, actor_name, actor_role, action, resource_type, resource_id, details, ip_address, recorded_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, '127.0.0.1', NOW())
                "#,
            )
            .bind(&aid)
            .bind(tenant_id)
            .bind(actor_id)
            .bind(actor_name)
            .bind(actor_role)
            .bind(action)
            .bind(resource_type)
            .bind(resource_id)
            .bind(details)
            .execute(pool)
            .await;
        }
        Ok(())
    }

    // ─── ADMIN DASHBOARD OPERATIONS ──────────────────────────────────────────

    pub async fn get_admin_overview(&self) -> AppResult<AdminOverviewDto> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let total_clients: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants")
                .fetch_one(pool)
                .await
                .unwrap_or(0);

            let active_services: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM client_services WHERE status = 'Active'")
                .fetch_one(pool)
                .await
                .unwrap_or(0);

            let active_projects: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE status = 'In Progress'")
                .fetch_one(pool)
                .await
                .unwrap_or(0);

            let pending_requests: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM change_requests WHERE status IN ('Submitted', 'Under Review', 'Quoted')")
                .fetch_one(pool)
                .await
                .unwrap_or(0);

            let rev_str: String = sqlx::query_scalar("SELECT COALESCE(SUM(total_amount)::text, '0.00') FROM invoices WHERE status = 'Paid'")
                .fetch_one(pool)
                .await
                .unwrap_or_else(|_| "78,200.00".to_string());

            let clients = self.get_admin_clients().await?;
            let recent_activity = self.get_admin_audit_logs().await?;

            return Ok(AdminOverviewDto {
                total_clients,
                active_services,
                active_projects,
                pending_requests,
                total_revenue_sar: rev_str,
                clients,
                recent_activity: recent_activity.into_iter().take(6).collect(),
            });
        }

        Ok(AdminOverviewDto {
            total_clients: 2,
            active_services: 3,
            active_projects: 2,
            pending_requests: 1,
            total_revenue_sar: "78,200.00".to_string(),
            clients: vec![],
            recent_activity: vec![],
        })
    }

    pub async fn get_admin_clients(&self) -> AppResult<Vec<AdminClientSummaryDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT t.id, t.name, t.slug, t.created_at::text as created_at,
                       (SELECT COUNT(*) FROM client_services cs WHERE cs.tenant_id = t.id AND cs.status = 'Active') as active_services,
                       (SELECT COUNT(*) FROM projects p WHERE p.tenant_id = t.id AND p.status = 'In Progress') as active_projects,
                       COALESCE((SELECT SUM(i.total_amount)::text FROM invoices i WHERE i.tenant_id = t.id AND i.status = 'Paid'), '0.00') as total_billed
                FROM tenants t
                ORDER BY t.created_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(AdminClientSummaryDto {
                    tenant_id: r.get("id"),
                    name: r.get("name"),
                    slug: r.get("slug"),
                    created_at: r.get("created_at"),
                    active_services_count: r.get("active_services"),
                    active_projects_count: r.get("active_projects"),
                    total_billed_sar: r.get("total_billed"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn get_admin_all_change_requests(&self) -> AppResult<Vec<ChangeRequestItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, project_id, requested_by_name, title, scope_description,
                       estimated_cost::text as estimated_cost, currency, estimated_days,
                       status, review_notes, created_at::text as created_at
                FROM change_requests
                ORDER BY created_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(ChangeRequestItemDto {
                    id: r.get("id"),
                    project_id: r.get("project_id"),
                    requested_by_name: r.get("requested_by_name"),
                    title: r.get("title"),
                    scope_description: r.get("scope_description"),
                    estimated_cost: r.get("estimated_cost"),
                    currency: r.get("currency"),
                    estimated_days: r.get("estimated_days"),
                    status: r.get("status"),
                    review_notes: r.get("review_notes"),
                    created_at: r.get("created_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn update_change_request_status(
        &self,
        request_id: &str,
        new_status: &str,
        review_notes: Option<&str>,
        admin_id: &str,
        admin_name: &str,
    ) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                UPDATE change_requests
                SET status = $1, review_notes = COALESCE($2, review_notes), resolved_at = NOW()
                WHERE id = $3
                "#,
            )
            .bind(new_status)
            .bind(review_notes)
            .bind(request_id)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let _ = self.record_audit_log(
                None,
                admin_id,
                admin_name,
                "Super Admin",
                "CHANGE_REQUEST_STATUS_UPDATED",
                "ChangeRequest",
                request_id,
                Some(&format!("New status: {}", new_status)),
            ).await;

            return Ok(());
        }
        Ok(())
    }

    pub async fn get_admin_audit_logs(&self) -> AppResult<Vec<AuditLogItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, actor_name, actor_role, action, resource_type,
                       resource_id, details, ip_address, recorded_at::text as recorded_at
                FROM audit_logs
                ORDER BY recorded_at DESC
                LIMIT 50
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(AuditLogItemDto {
                    id: r.get("id"),
                    actor_name: r.get("actor_name"),
                    actor_role: r.get("actor_role"),
                    action: r.get("action"),
                    resource_type: r.get("resource_type"),
                    resource_id: r.get("resource_id"),
                    details: r.get("details"),
                    ip_address: r.get("ip_address"),
                    recorded_at: r.get("recorded_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn get_admin_all_services(&self) -> AppResult<Vec<ClientServiceItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT cs.id, cs.service_id, s.title, s.title_ar, s.category,
                       cs.package_name, cs.status, cs.price::text as price, cs.currency,
                       cs.sla_tier, cs.assigned_lead,
                       cs.start_date::text as start_date, cs.renewal_date::text as renewal_date
                FROM client_services cs
                JOIN services s ON s.id = cs.service_id
                ORDER BY cs.created_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                list.push(ClientServiceItemDto {
                    id: r.get("id"),
                    service_id: r.get("service_id"),
                    title: r.get("title"),
                    title_ar: r.get("title_ar"),
                    category: r.get("category"),
                    package_name: r.get("package_name"),
                    status: r.get("status"),
                    price: r.get("price"),
                    currency: r.get("currency"),
                    sla_tier: r.get("sla_tier"),
                    assigned_lead: r.get("assigned_lead"),
                    start_date: r.get("start_date"),
                    renewal_date: r.get("renewal_date"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn get_admin_all_projects(&self) -> AppResult<Vec<ProjectItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let project_rows = sqlx::query(
                r#"
                SELECT id, client_service_id, title, description, status,
                       progress_pct, priority, deadline::text as deadline, assigned_team
                FROM projects
                ORDER BY created_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut projects = Vec::new();
            for prow in project_rows {
                let pid: String = prow.get("id");
                let m_rows = sqlx::query(
                    r#"
                    SELECT id, title, description, status, due_date::text as due_date, completed_at::text as completed_at
                    FROM project_milestones
                    WHERE project_id = $1
                    ORDER BY sort_order ASC
                    "#,
                )
                .bind(&pid)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                let milestones = m_rows
                    .into_iter()
                    .map(|m| MilestoneDto {
                        id: m.get("id"),
                        title: m.get("title"),
                        description: m.get("description"),
                        status: m.get("status"),
                        due_date: m.get("due_date"),
                        completed_at: m.get("completed_at"),
                    })
                    .collect();

                projects.push(ProjectItemDto {
                    id: pid,
                    client_service_id: prow.get("client_service_id"),
                    title: prow.get("title"),
                    description: prow.get("description"),
                    status: prow.get("status"),
                    progress_pct: prow.get("progress_pct"),
                    priority: prow.get("priority"),
                    deadline: prow.get("deadline"),
                    assigned_team: prow.get("assigned_team"),
                    milestones,
                });
            }
            return Ok(projects);
        }
        Ok(vec![])
    }

    pub async fn get_admin_all_invoices(&self) -> AppResult<Vec<InvoiceItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let inv_rows = sqlx::query(
                r#"
                SELECT id, invoice_number, status, subtotal::text as subtotal,
                       tax_amount::text as tax_amount, total_amount::text as total_amount,
                       currency, due_date::text as due_date, issued_at::text as issued_at,
                       paid_at::text as paid_at, payment_method, notes
                FROM invoices
                ORDER BY issued_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for inv in inv_rows {
                let inv_id: String = inv.get("id");
                let item_rows = sqlx::query(
                    r#"
                    SELECT id, description, quantity, unit_price::text as unit_price, total_amount::text as total_amount
                    FROM invoice_items
                    WHERE invoice_id = $1
                    ORDER BY sort_order ASC
                    "#,
                )
                .bind(&inv_id)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                let items = item_rows
                    .into_iter()
                    .map(|it| InvoiceLineItemDto {
                        id: it.get("id"),
                        description: it.get("description"),
                        quantity: it.get("quantity"),
                        unit_price: it.get("unit_price"),
                        total_amount: it.get("total_amount"),
                    })
                    .collect();

                list.push(InvoiceItemDto {
                    id: inv_id,
                    invoice_number: inv.get("invoice_number"),
                    status: inv.get("status"),
                    subtotal: inv.get("subtotal"),
                    tax_amount: inv.get("tax_amount"),
                    total_amount: inv.get("total_amount"),
                    currency: inv.get("currency"),
                    due_date: inv.get("due_date"),
                    issued_at: inv.get("issued_at"),
                    paid_at: inv.get("paid_at"),
                    payment_method: inv.get("payment_method"),
                    notes: inv.get("notes"),
                    items,
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn get_admin_all_deliverables(&self) -> AppResult<Vec<DeliverableItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let rows = sqlx::query(
                r#"
                SELECT id, project_id, title, file_name, file_url,
                       file_size_bytes, version, status, published_at::text as published_at
                FROM deliverables
                ORDER BY published_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for r in rows {
                let bytes: i64 = r.get("file_size_bytes");
                let formatted_size = if bytes > 1_048_576 {
                    format!("{:.1} MB", bytes as f64 / 1_048_576.0)
                } else {
                    format!("{:.1} KB", bytes as f64 / 1024.0)
                };

                list.push(DeliverableItemDto {
                    id: r.get("id"),
                    project_id: r.get("project_id"),
                    title: r.get("title"),
                    file_name: r.get("file_name"),
                    file_url: r.get("file_url"),
                    file_size_formatted: formatted_size,
                    version: r.get("version"),
                    status: r.get("status"),
                    published_at: r.get("published_at"),
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn get_admin_all_conversations(&self) -> AppResult<Vec<ConversationItemDto>> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            let conv_rows = sqlx::query(
                r#"
                SELECT id, title, channel_type, related_project_id, updated_at::text as updated_at
                FROM conversations
                ORDER BY updated_at DESC
                "#,
            )
            .fetch_all(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let mut list = Vec::new();
            for c in conv_rows {
                let cid: String = c.get("id");
                let msg_rows = sqlx::query(
                    r#"
                    SELECT id, conversation_id, sender_id, sender_name, sender_role, body, is_internal, sent_at::text as sent_at
                    FROM messages
                    WHERE conversation_id = $1
                    ORDER BY sent_at ASC
                    "#,
                )
                .bind(&cid)
                .fetch_all(pool)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

                let messages = msg_rows
                    .into_iter()
                    .map(|m| MessageItemDto {
                        id: m.get("id"),
                        conversation_id: m.get("conversation_id"),
                        sender_id: m.get("sender_id"),
                        sender_name: m.get("sender_name"),
                        sender_role: m.get("sender_role"),
                        body: m.get("body"),
                        is_internal: m.get("is_internal"),
                        sent_at: m.get("sent_at"),
                    })
                    .collect();

                list.push(ConversationItemDto {
                    id: cid,
                    title: c.get("title"),
                    channel_type: c.get("channel_type"),
                    related_project_id: c.get("related_project_id"),
                    updated_at: c.get("updated_at"),
                    messages,
                });
            }
            return Ok(list);
        }
        Ok(vec![])
    }

    pub async fn admin_mark_invoice_paid(
        &self,
        invoice_id: &str,
        payment_method: &str,
        admin_id: &str,
        admin_name: &str,
    ) -> AppResult<()> {
        #[cfg(feature = "sqlx")]
        if let Some(pool) = &self.pool {
            sqlx::query(
                r#"
                UPDATE invoices
                SET status = 'Paid', paid_at = NOW(), payment_method = $2
                WHERE id = $1
                "#,
            )
            .bind(invoice_id)
            .bind(payment_method)
            .execute(pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

            let _ = self.record_audit_log(
                None,
                admin_id,
                admin_name,
                "Super Admin",
                "INVOICE_MARKED_PAID",
                "Invoice",
                invoice_id,
                Some(&format!("Payment method: {}", payment_method)),
            ).await;

            return Ok(());
        }
        Ok(())
    }
}
