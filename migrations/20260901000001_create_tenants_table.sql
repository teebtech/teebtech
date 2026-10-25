-- ==============================================================================
-- Migration: 20260901000001_create_tenants_table.sql
-- Description: Creates the tenants table and index for multi-tenant isolation
-- ==============================================================================

CREATE TABLE IF NOT EXISTS tenants (
    id VARCHAR(64) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_tenants_slug ON tenants(slug);
