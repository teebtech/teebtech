//! # Zero-Trust Multi-Tenant Isolation Integration Tests
//!
//! Proves strict boundary isolation between Client A (Acme Corp) and Client B (Vertex Systems).
//! Verifies that under no circumstance can Client A read or mutate Client B records.

use database::DatabasePool;

#[tokio::test]
async fn test_tenant_isolation_acme_vs_vertex() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
    });

    let pool = match DatabasePool::connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return, // Skip if Postgres not accessible in test runner
    };

    let portal = pool.portal_repository();

    let tenant_acme = "tenant_acme_corp";
    let tenant_vertex = "tenant_vertex_sys";

    // 1. Verify Client Services Isolation
    let acme_services = portal.get_client_services(tenant_acme).await.unwrap();
    assert!(!acme_services.is_empty(), "Acme should have active services");
    for s in &acme_services {
        assert_ne!(s.id, "cs_vertex_01", "Acme MUST NOT see Vertex services");
    }

    let vertex_services = portal.get_client_services(tenant_vertex).await.unwrap();
    assert_eq!(vertex_services.len(), 1);
    assert_eq!(vertex_services[0].id, "cs_vertex_01");

    // 2. Verify Projects Isolation
    let acme_projects = portal.get_client_projects(tenant_acme).await.unwrap();
    for p in &acme_projects {
        assert_ne!(p.id, "prj_vertex_01", "Acme MUST NOT see Vertex projects");
        assert_ne!(p.title, "Vertex Predictive Fraud Engine");
    }

    let vertex_projects = portal.get_client_projects(tenant_vertex).await.unwrap();
    assert_eq!(vertex_projects.len(), 1);
    assert_eq!(vertex_projects[0].id, "prj_vertex_01");

    // 3. Verify Invoices Isolation
    let acme_invoices = portal.get_client_invoices(tenant_acme).await.unwrap();
    for inv in &acme_invoices {
        assert_ne!(inv.id, "inv_vertex_01", "Acme MUST NOT see Vertex invoices");
        assert_ne!(inv.invoice_number, "INV-2026-0099");
    }

    let vertex_invoices = portal.get_client_invoices(tenant_vertex).await.unwrap();
    assert_eq!(vertex_invoices.len(), 1);
    assert_eq!(vertex_invoices[0].invoice_number, "INV-2026-0099");

    // 4. Verify ZERO-TRUST Internal Message Isolation:
    // Client view (is_staff = false) MUST NOT receive internal notes
    let client_convs = portal.get_client_conversations(tenant_acme, false).await.unwrap();
    assert!(!client_convs.is_empty());
    for conv in &client_convs {
        for msg in &conv.messages {
            assert!(!msg.is_internal, "Client view MUST NOT receive internal notes!");
            assert_ne!(msg.id, "msg_acme_03", "Internal note MUST NOT be returned to client");
        }
    }

    // Staff view (is_staff = true) DOES receive internal notes for triage
    let staff_convs = portal.get_client_conversations(tenant_acme, true).await.unwrap();
    let has_internal = staff_convs
        .iter()
        .flat_map(|c| &c.messages)
        .any(|m| m.is_internal && m.id == "msg_acme_03");
    assert!(has_internal, "Staff view MUST receive internal notes for operational support");
}

#[tokio::test]
async fn test_cross_tenant_change_request_idor_protection() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
    });

    let pool = match DatabasePool::connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return,
    };
    let portal = pool.portal_repository();

    let tenant_vertex = "tenant_vertex_sys";
    let acme_change_request_id = "cr_acme_01";

    // Vertex attempts to approve Acme's change request (IDOR attack)
    let result = portal
        .client_decide_change_request(
            tenant_vertex,
            acme_change_request_id,
            true,
            "usr_vertex_01",
            "Tariq Al-Mansoor",
        )
        .await;

    assert!(result.is_err(), "Cross-tenant change request approval MUST be rejected!");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("not found or unauthorized"),
        "Error message should cite unauthorized/not found: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_cross_tenant_deliverable_download_idor_protection() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
    });

    let pool = match DatabasePool::connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return,
    };
    let portal = pool.portal_repository();

    let tenant_vertex = "tenant_vertex_sys";
    let acme_deliverable_id = "deliv_acme_01";

    // Vertex attempts to access Acme's deliverable metadata / download stream
    let result = portal
        .get_deliverable_for_download(tenant_vertex, acme_deliverable_id)
        .await
        .expect("Query execution must succeed");

    assert!(result.is_none(), "Cross-tenant deliverable download MUST be filtered out (None)!");

    // Acme accessing own deliverable MUST succeed
    let acme_result = portal
        .get_deliverable_for_download("tenant_acme_corp", acme_deliverable_id)
        .await
        .expect("Query execution must succeed");
    assert!(acme_result.is_some(), "Acme must be authorized for own deliverable");
    let deliv = acme_result.unwrap();
    assert_eq!(deliv.id, acme_deliverable_id);
}

#[tokio::test]
async fn test_client_package_order_and_notification_workflow() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:5433/teebtech_dev".to_string()
    });

    let pool = match DatabasePool::connect(&db_url).await {
        Ok(p) => p,
        Err(_) => return,
    };
    let portal = pool.portal_repository();

    let tenant_acme = "tenant_acme_corp";
    let user_id = "usr_client_01";
    let user_name = "Sarah Jenkins";

    // 1. Order a service package
    let cr_id = portal
        .order_service_package(
            tenant_acme,
            user_id,
            user_name,
            "srv_microservices",
            "Enterprise Architecture Retainer",
            "Need immediate production readiness review for Q4",
        )
        .await
        .expect("Ordering service package must succeed");

    assert!(cr_id.starts_with("cr_"));

    // 2. Verify request is persisted under Acme tenant
    let requests = portal.get_client_change_requests(tenant_acme).await.unwrap();
    let found = requests.iter().find(|r| r.id == cr_id);
    assert!(found.is_some(), "New change request must be persisted in tenant records");
    assert_eq!(found.unwrap().status, "Submitted");

    // 3. Create a client conversation thread
    let conv_id = portal
        .create_client_conversation(
            tenant_acme,
            user_id,
            user_name,
            "Architecture Review Q4 Scope Discussion",
            "Support",
            "Hello engineering team, please review our architecture timeline.",
        )
        .await
        .expect("Creating conversation must succeed");

    assert!(conv_id.starts_with("conv_"));

    let convs = portal.get_client_conversations(tenant_acme, false).await.unwrap();
    let conv_found = convs.iter().find(|c| c.id == conv_id);
    assert!(conv_found.is_some(), "New conversation must be retrievable by tenant");
    assert_eq!(conv_found.unwrap().messages.len(), 1);
    assert_eq!(
        conv_found.unwrap().messages[0].body,
        "Hello engineering team, please review our architecture timeline."
    );

    // 4. Mark notifications read
    let mark_res = portal.mark_client_notifications_read(tenant_acme).await;
    assert!(mark_res.is_ok(), "Marking notifications read must succeed");

    let overview = portal.get_client_overview(tenant_acme, user_id).await.unwrap();
    assert_eq!(overview.unread_notifications_count, 0, "Unread notifications must be 0 after marking read");
}

