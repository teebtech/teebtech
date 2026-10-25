-- ==============================================================================
-- Migration: 20261025000001_create_enterprise_portal_schema.sql
-- Description: Complete enterprise portal schema for multi-tenant service delivery,
--              projects, change requests, conversations, deliverables, invoices,
--              notifications, and immutable audit logs.
-- ==============================================================================

-- 1. Services Catalog
CREATE TABLE IF NOT EXISTS services (
    id VARCHAR(64) PRIMARY KEY,
    slug VARCHAR(64) NOT NULL UNIQUE,
    title VARCHAR(150) NOT NULL,
    title_ar VARCHAR(150) NOT NULL,
    category VARCHAR(64) NOT NULL,
    description TEXT NOT NULL,
    description_ar TEXT NOT NULL,
    icon VARCHAR(64) NOT NULL DEFAULT 'briefcase',
    status VARCHAR(30) NOT NULL DEFAULT 'Published', -- 'Draft', 'Published', 'Archived'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_services_category ON services(category);

-- 2. Service Packages & Pricing
CREATE TABLE IF NOT EXISTS service_packages (
    id VARCHAR(64) PRIMARY KEY,
    service_id VARCHAR(64) NOT NULL REFERENCES services(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    name_ar VARCHAR(100) NOT NULL,
    price NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(10) NOT NULL DEFAULT 'SAR',
    billing_period VARCHAR(30) NOT NULL DEFAULT 'One-Time', -- 'One-Time', 'Monthly', 'Annual'
    features_json JSONB NOT NULL DEFAULT '[]'::jsonb
);
CREATE INDEX IF NOT EXISTS idx_service_packages_service ON service_packages(service_id);

-- 3. Purchased Client Services (Tenant Service Instances)
CREATE TABLE IF NOT EXISTS client_services (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    service_id VARCHAR(64) NOT NULL REFERENCES services(id) ON DELETE RESTRICT,
    package_name VARCHAR(100) NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'Active', -- 'Pending', 'Active', 'Paused', 'Completed', 'Cancelled'
    price NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(10) NOT NULL DEFAULT 'SAR',
    sla_tier VARCHAR(50) NOT NULL DEFAULT 'Standard 24/7',
    assigned_lead VARCHAR(100) NOT NULL DEFAULT 'Engineering Team Lead',
    start_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    renewal_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_client_services_tenant ON client_services(tenant_id);
CREATE INDEX IF NOT EXISTS idx_client_services_status ON client_services(status);

-- 4. Projects
CREATE TABLE IF NOT EXISTS projects (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    client_service_id VARCHAR(64) REFERENCES client_services(id) ON DELETE SET NULL,
    title VARCHAR(150) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'In Progress', -- 'Planning', 'In Progress', 'In Review', 'Completed', 'Blocked'
    progress_pct INT NOT NULL DEFAULT 0 CHECK (progress_pct >= 0 AND progress_pct <= 100),
    priority VARCHAR(20) NOT NULL DEFAULT 'Normal', -- 'Low', 'Normal', 'High', 'Urgent'
    deadline TIMESTAMPTZ,
    assigned_team VARCHAR(100) NOT NULL DEFAULT 'TeebTech Core Squad',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_projects_tenant ON projects(tenant_id);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);

-- 5. Project Milestones
CREATE TABLE IF NOT EXISTS project_milestones (
    id VARCHAR(64) PRIMARY KEY,
    project_id VARCHAR(64) NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title VARCHAR(150) NOT NULL,
    description TEXT NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'Pending', -- 'Pending', 'In Progress', 'Completed'
    due_date TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_project_milestones_proj ON project_milestones(project_id);

-- 6. Service Requests & Scope Change Requests
CREATE TABLE IF NOT EXISTS change_requests (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    client_service_id VARCHAR(64) REFERENCES client_services(id) ON DELETE SET NULL,
    project_id VARCHAR(64) REFERENCES projects(id) ON DELETE SET NULL,
    requested_by_user_id VARCHAR(64) NOT NULL,
    requested_by_name VARCHAR(100) NOT NULL,
    title VARCHAR(150) NOT NULL,
    scope_description TEXT NOT NULL,
    estimated_cost NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(10) NOT NULL DEFAULT 'SAR',
    estimated_days INT NOT NULL DEFAULT 0,
    status VARCHAR(30) NOT NULL DEFAULT 'Submitted', -- 'Submitted', 'Under Review', 'Quoted', 'Approved', 'Rejected', 'Implemented'
    review_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);
CREATE INDEX IF NOT EXISTS idx_change_requests_tenant ON change_requests(tenant_id);
CREATE INDEX IF NOT EXISTS idx_change_requests_status ON change_requests(status);

-- 7. Communication Channels & Conversations
CREATE TABLE IF NOT EXISTS conversations (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    title VARCHAR(150) NOT NULL,
    channel_type VARCHAR(50) NOT NULL DEFAULT 'General Support', -- 'General Support', 'Project', 'Billing', 'Emergency'
    related_project_id VARCHAR(64) REFERENCES projects(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_conversations_tenant ON conversations(tenant_id);

-- 8. Messages (With Strict Internal Notes Flag)
CREATE TABLE IF NOT EXISTS messages (
    id VARCHAR(64) PRIMARY KEY,
    conversation_id VARCHAR(64) NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id VARCHAR(64) NOT NULL,
    sender_name VARCHAR(100) NOT NULL,
    sender_role VARCHAR(50) NOT NULL, -- 'Client', 'Staff', 'Admin'
    body TEXT NOT NULL,
    is_internal BOOLEAN NOT NULL DEFAULT FALSE, -- ZERO-TRUST: Internal notes NEVER sent to client
    sent_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_messages_conv ON messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_messages_internal ON messages(is_internal);

-- 9. Deliverables & Access-Controlled Files
CREATE TABLE IF NOT EXISTS deliverables (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    project_id VARCHAR(64) REFERENCES projects(id) ON DELETE CASCADE,
    title VARCHAR(150) NOT NULL,
    file_name VARCHAR(150) NOT NULL,
    file_url VARCHAR(255) NOT NULL,
    file_size_bytes BIGINT NOT NULL DEFAULT 0,
    version VARCHAR(20) NOT NULL DEFAULT 'v1.0.0',
    status VARCHAR(30) NOT NULL DEFAULT 'Published', -- 'Draft', 'Published', 'Approved', 'Archived'
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_deliverables_tenant ON deliverables(tenant_id);

-- 10. Invoices & Billing
CREATE TABLE IF NOT EXISTS invoices (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    invoice_number VARCHAR(50) NOT NULL UNIQUE,
    status VARCHAR(30) NOT NULL DEFAULT 'Issued', -- 'Draft', 'Issued', 'Paid', 'Overdue', 'Cancelled'
    subtotal NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    tax_amount NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    total_amount NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    currency VARCHAR(10) NOT NULL DEFAULT 'SAR',
    due_date TIMESTAMPTZ NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    paid_at TIMESTAMPTZ,
    payment_method VARCHAR(50) DEFAULT 'Bank Wire Transfer',
    notes TEXT
);
CREATE INDEX IF NOT EXISTS idx_invoices_tenant ON invoices(tenant_id);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);

-- 11. Invoice Line Items
CREATE TABLE IF NOT EXISTS invoice_items (
    id VARCHAR(64) PRIMARY KEY,
    invoice_id VARCHAR(64) NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description VARCHAR(200) NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    unit_price NUMERIC(12, 2) NOT NULL DEFAULT 0.00,
    total_amount NUMERIC(12, 2) NOT NULL DEFAULT 0.00
);
CREATE INDEX IF NOT EXISTS idx_invoice_items_inv ON invoice_items(invoice_id);

-- 12. Tenant Notifications
CREATE TABLE IF NOT EXISTS notifications (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64) NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    recipient_user_id VARCHAR(64),
    title VARCHAR(150) NOT NULL,
    title_ar VARCHAR(150) NOT NULL,
    body TEXT NOT NULL,
    body_ar TEXT NOT NULL,
    category VARCHAR(50) NOT NULL DEFAULT 'System', -- 'Service', 'Project', 'Billing', 'Message'
    link_url VARCHAR(255),
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_notifications_tenant ON notifications(tenant_id);
CREATE INDEX IF NOT EXISTS idx_notifications_read ON notifications(is_read);

-- 13. Immutable Security Audit Logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id VARCHAR(64) PRIMARY KEY,
    tenant_id VARCHAR(64), -- Can be NULL for platform-wide admin operations
    actor_id VARCHAR(64) NOT NULL,
    actor_name VARCHAR(100) NOT NULL,
    actor_role VARCHAR(50) NOT NULL,
    action VARCHAR(100) NOT NULL, -- e.g., 'SERVICE_PURCHASED', 'CHANGE_REQUEST_APPROVED', 'DELIVERABLE_PUBLISHED'
    resource_type VARCHAR(50) NOT NULL,
    resource_id VARCHAR(64) NOT NULL,
    details TEXT,
    ip_address VARCHAR(45) NOT NULL DEFAULT '127.0.0.1',
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_audit_logs_tenant ON audit_logs(tenant_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);
CREATE INDEX IF NOT EXISTS idx_audit_logs_time ON audit_logs(recorded_at DESC);
