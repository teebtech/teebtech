-- ==============================================================================
-- Migration: 20261025000002_seed_enterprise_portal_data.sql
-- Description: Realistic seed data for TeebTech Enterprise Services,
--              Client A (Acme Corp), Client B (Vertex Systems),
--              and Platform Admin with strict isolation boundaries.
-- ==============================================================================

-- 1. Ensure Tenants Exist
INSERT INTO tenants (id, name, slug)
VALUES
  ('tenant_acme_corp', 'Acme Corporation', 'acme-corp'),
  ('tenant_vertex_sys', 'Vertex Systems', 'vertex-sys')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name;

-- 2. Ensure Users Exist (Password is Password123!)
INSERT INTO users (id, email, password_hash, display_name)
VALUES
  ('usr_admin_001', 'admin@teebtech.com', '$2b$12$7mMBCQjsip9rq3s0MN51I.q4CGUv8.3ezjxfKxnIaIK.FXo6lr9nS', 'Platform Admin'),
  ('usr_client_001', 'client@teebtech.com', '$2b$12$7mMBCQjsip9rq3s0MN51I.q4CGUv8.3ezjxfKxnIaIK.FXo6lr9nS', 'Sarah Jenkins'),
  ('usr_demo_001', 'demo@teebtech.com', '$2b$12$7mMBCQjsip9rq3s0MN51I.q4CGUv8.3ezjxfKxnIaIK.FXo6lr9nS', 'Tariq Al-Mansoor')
ON CONFLICT (id) DO UPDATE SET password_hash = EXCLUDED.password_hash, display_name = EXCLUDED.display_name;

-- 3. Tenant Memberships
INSERT INTO tenant_members (id, tenant_id, email, name, role)
VALUES
  ('mem_admin_001', 'tenant_acme_corp', 'admin@teebtech.com', 'Platform Admin', 'Owner'),
  ('mem_client_001', 'tenant_acme_corp', 'client@teebtech.com', 'Sarah Jenkins', 'Owner'),
  ('mem_demo_001', 'tenant_vertex_sys', 'demo@teebtech.com', 'Tariq Al-Mansoor', 'Owner')
ON CONFLICT (id) DO UPDATE SET tenant_id = EXCLUDED.tenant_id, name = EXCLUDED.name, role = EXCLUDED.role;

-- 4. Service Catalog
INSERT INTO services (id, slug, title, title_ar, category, description, description_ar, icon, status)
VALUES
  ('srv_web_platform', 'cloud-web-platform', 'Enterprise Web Platform', 'المنصة السحابية المؤسسية المتكاملة', 'Software Development', 'Custom mission-critical web applications with distributed architecture, high availability, and sub-second latency.', 'تطوير تطبيقات الويب الحيوية بمعمارية موزعة عالية التوافر ومعدل استجابة فائق السرعة.', 'code', 'Published'),
  ('srv_mobile_suite', 'cross-platform-mobile', 'Native & Cross-Platform Mobile Suite', 'حزمة تطبيقات الهواتف الذكية المتقدمة', 'Mobile Development', 'iOS and Android applications engineered for fluid UX, offline sync, biometric security, and enterprise integrations.', 'تطبيقات iOS وأندرويد بأعلى معايير تجربة المستخدم ومزامنة البيانات وتكامل الأنظمة.', 'briefcase', 'Published'),
  ('srv_cloud_devops', 'cloud-devops-infra', 'Cloud Infrastructure & DevOps CI/CD', 'البنية التحتية السحابية وأتمتة العمليات', 'Cloud & DevOps', 'Automated cloud foundation on AWS/Azure with zero-downtime blue/green deployment pipelines and Kubernetes orchestration.', 'بنية تحتية سحابية مجهزة على AWS/Azure مع خطوط نشر مؤتمتة وإدارة حاويات Kubernetes.', 'server', 'Published'),
  ('srv_cyber_security', 'zero-trust-cybersecurity', 'Zero-Trust Cybersecurity & Penetration Testing', 'الأمن السيبراني وبنية الثقة الصفرية', 'Cybersecurity', 'End-to-end security audits, threat modeling, automated vulnerability monitoring, and regulatory compliance (NCA, ISO 27001).', 'تدقيق أمني شامل، نمذجة التهديدات، فحص الثغرات الدائم ومطابقة المعايير الوطنية والدولية.', 'shield', 'Published'),
  ('srv_ai_data', 'ai-big-data-pipeline', 'AI Solutions & Big Data Pipeline', 'حلول الذكاء الاصطناعي ومعالجة البيانات', 'AI & Data', 'Custom predictive AI models, real-time data streaming pipelines, analytics dashboards, and automated decision engines.', 'نماذج ذكاء اصطناعي تنبؤية، خطوط تدفق بيانات لحظية ولوحات تحكم متقدمة للأعمال.', 'cpu', 'Published'),
  ('srv_managed_it', '24-7-managed-sre', '24/7 Managed SRE & Infrastructure SLA', 'الإدارة السحابية ودعم SRE على مدار الساعة', 'Managed Services', 'Round-the-clock incident response, proactive patch management, automated failover, and strict 99.99% uptime SLAs.', 'استجابة فورية للحوادث، ترقيع أمني استباقي، استرداد تلقائي للأعطال وضمان تشغيل 99.99%.', 'settings', 'Published')
ON CONFLICT (id) DO NOTHING;

-- 5. Service Packages
INSERT INTO service_packages (id, service_id, name, name_ar, price, currency, billing_period, features_json)
VALUES
  ('pkg_web_starter', 'srv_web_platform', 'Standard Enterprise', 'المؤسسي القياسي', 35000.00, 'SAR', 'One-Time', '["Custom UI/UX Design", "Rust & Leptos SSR Engine", "PostgreSQL Database Architecture", "Role-Based Access Control", "3 Months Post-Launch SLA"]'::jsonb),
  ('pkg_web_pro', 'srv_web_platform', 'Flagship Platform', 'المنصة الرائدة الكاملة', 68000.00, 'SAR', 'One-Time', '["High-Scale Distributed Microservices", "Multi-Tenant Architecture", "Real-Time WebSocket Engine", "Automated Load Testing", "12 Months Dedicated SRE Support"]'::jsonb),
  ('pkg_mobile_starter', 'srv_mobile_suite', 'iOS + Android Pro', 'تطبيقات الهواتف المزدوجة', 42000.00, 'SAR', 'One-Time', '["Native Performance", "Offline Data Sync", "Push Notification Architecture", "App Store & Play Store Publishing"]'::jsonb),
  ('pkg_cloud_devops', 'srv_cloud_devops', 'Cloud DevOps Architecture', 'هندسة السحابة وعمليات CI/CD', 28000.00, 'SAR', 'One-Time', '["Terraform Infrastructure-as-Code", "Multi-Region Cloud Setup", "Automated CI/CD Pipeline", "24/7 Health Monitoring"]'::jsonb),
  ('pkg_cyber_security', 'srv_cyber_security', 'Zero-Trust Audit & Hardening', 'التدقيق الأمني والتحصين الشامل', 22000.00, 'SAR', 'One-Time', '["Comprehensive Black-Box Pentest", "NCA ECC Compliance Review", "API Security Hardening", "Executive Mitigation Report"]'::jsonb),
  ('pkg_managed_it', 'srv_managed_it', 'Enterprise 24/7 SRE SLA', 'عقد الدعم المدار المؤسسي 24/7', 9500.00, 'SAR', 'Monthly', '["Dedicated Senior DevOps Lead", "15-Minute Incident Response SLA", "Weekly Security Backups", "Continuous Health Checks"]'::jsonb)
ON CONFLICT (id) DO NOTHING;

-- 6. Client A (Acme Corp) - Purchased Services
INSERT INTO client_services (id, tenant_id, service_id, package_name, status, price, currency, sla_tier, assigned_lead, start_date, renewal_date)
VALUES
  ('cs_acme_01', 'tenant_acme_corp', 'srv_web_platform', 'Flagship Platform', 'Active', 68000.00, 'SAR', 'Platinum 24/7 (15 min SLA)', 'Eng. Omar Farooq (Principal Architect)', NOW() - INTERVAL '45 days', NOW() + INTERVAL '320 days'),
  ('cs_acme_02', 'tenant_acme_corp', 'srv_cloud_devops', 'Cloud DevOps Architecture', 'Active', 28000.00, 'SAR', 'Enterprise Gold (30 min SLA)', 'DevOps Lead: Zaid Al-Husseini', NOW() - INTERVAL '30 days', NOW() + INTERVAL '335 days'),
  ('cs_acme_03', 'tenant_acme_corp', 'srv_cyber_security', 'Zero-Trust Audit & Hardening', 'Completed', 22000.00, 'SAR', 'Completed Project', 'SecOps Lead: Faisal Al-Mutairi', NOW() - INTERVAL '60 days', NOW() - INTERVAL '10 days')
ON CONFLICT (id) DO NOTHING;

-- 7. Client A (Acme Corp) - Projects
INSERT INTO projects (id, tenant_id, client_service_id, title, description, status, progress_pct, priority, deadline, assigned_team)
VALUES
  ('prj_acme_01', 'tenant_acme_corp', 'cs_acme_01', 'Acme Multi-Tenant B2B Platform', 'Core web application revamp featuring Rust/Leptos SSR, high-concurrency payment integrations, and low-latency database queries.', 'In Progress', 75, 'High', NOW() + INTERVAL '24 days', 'TeebTech Squad Alpha'),
  ('prj_acme_02', 'tenant_acme_corp', 'cs_acme_02', 'AWS Multi-Region Migration', 'Zero-downtime infrastructure migration to AWS Frankfurt with automated Terraform IaC and CloudWatch alerts.', 'In Progress', 90, 'Urgent', NOW() + INTERVAL '7 days', 'Cloud DevOps Engineering Team')
ON CONFLICT (id) DO NOTHING;

-- 8. Project Milestones (Acme Corp)
INSERT INTO project_milestones (id, project_id, title, description, status, due_date, completed_at)
VALUES
  ('ms_acme_01', 'prj_acme_01', 'System Architecture & Hexagonal Foundation', 'Domain boundaries, value objects, and database schemas completed.', 'Completed', NOW() - INTERVAL '30 days', NOW() - INTERVAL '28 days'),
  ('ms_acme_02', 'prj_acme_01', 'UI/UX Interactive Component Implementation', 'Design system alignment, responsive sidebar, Arabic/English RTL/LTR.', 'Completed', NOW() - INTERVAL '12 days', NOW() - INTERVAL '10 days'),
  ('ms_acme_03', 'prj_acme_01', 'Client Service Workflow & Invoicing Integration', 'Server-side transactional use cases and billing module reconciliation.', 'In Progress', NOW() + INTERVAL '10 days', NULL),
  ('ms_acme_04', 'prj_acme_01', 'Penetration Testing & Production Deployment', 'Security audit signoff, SSL provisioning, and production DNS switchover.', 'Pending', NOW() + INTERVAL '24 days', NULL)
ON CONFLICT (id) DO NOTHING;

-- 9. Change Requests (Acme Corp)
INSERT INTO change_requests (id, tenant_id, client_service_id, project_id, requested_by_user_id, requested_by_name, title, scope_description, estimated_cost, currency, estimated_days, status, review_notes)
VALUES
  ('cr_acme_01', 'tenant_acme_corp', 'cs_acme_01', 'prj_acme_01', 'usr_client_001', 'Sarah Jenkins', 'Additional Biometric MFA Verification', 'Requesting WebAuthn Passkeys and Hardware Token support for administrative actions.', 4500.00, 'SAR', 5, 'Quoted', 'Architectural impact assessed. Requires additional WebAuthn server credentials handler.'),
  ('cr_acme_02', 'tenant_acme_corp', 'cs_acme_01', 'prj_acme_01', 'usr_client_001', 'Sarah Jenkins', 'Automated Daily PDF Export Engine', 'Automated nightly export of client operational metrics formatted as PDF attachments.', 3200.00, 'SAR', 3, 'Approved', 'Approved by Client Owner. Scheduled for upcoming sprint delivery.')
ON CONFLICT (id) DO NOTHING;

-- 10. Conversations & Messages (Acme Corp)
INSERT INTO conversations (id, tenant_id, title, channel_type, related_project_id, updated_at)
VALUES
  ('conv_acme_01', 'tenant_acme_corp', 'Acme B2B Project Channel', 'Project', 'prj_acme_01', NOW() - INTERVAL '2 hours'),
  ('conv_acme_02', 'tenant_acme_corp', 'Billing & SRE Support', 'General Support', NULL, NOW() - INTERVAL '1 day')
ON CONFLICT (id) DO NOTHING;

INSERT INTO messages (id, conversation_id, sender_id, sender_name, sender_role, body, is_internal, sent_at)
VALUES
  ('msg_acme_01', 'conv_acme_01', 'usr_client_001', 'Sarah Jenkins', 'Client', 'Good morning team! We reviewed the latest milestone progress. The new responsive sidebar looks fantastic.', FALSE, NOW() - INTERVAL '4 hours'),
  ('msg_acme_02', 'conv_acme_01', 'usr_admin_001', 'Eng. Omar Farooq', 'Staff', 'Thank you Sarah! Milestone 2 has been formally signed off. We are currently testing the transactional invoicing and change request flows.', FALSE, NOW() - INTERVAL '3 hours'),
  ('msg_acme_03', 'conv_acme_01', 'usr_admin_001', 'Eng. Omar Farooq', 'Staff', 'INTERNAL NOTE: Ensure database connection pool max connections is checked before running load tests for Acme.', TRUE, NOW() - INTERVAL '2 hours')
ON CONFLICT (id) DO NOTHING;

-- 11. Deliverables (Acme Corp)
INSERT INTO deliverables (id, tenant_id, project_id, title, file_name, file_url, file_size_bytes, version, status, published_at)
VALUES
  ('deliv_acme_01', 'tenant_acme_corp', 'prj_acme_01', 'Acme Platform Architectural Blueprint', 'Acme_Platform_Architecture_v1.0.pdf', '/downloads/deliverables/Acme_Architecture.pdf', 3450000, 'v1.0.0', 'Published', NOW() - INTERVAL '25 days'),
  ('deliv_acme_02', 'tenant_acme_corp', 'prj_acme_01', 'NCA Cybersecurity Pentest Signoff', 'ZeroTrust_Cybersecurity_Audit_Signoff.pdf', '/downloads/deliverables/Audit_Signoff.pdf', 1820000, 'v1.2.0', 'Approved', NOW() - INTERVAL '10 days')
ON CONFLICT (id) DO NOTHING;

-- 12. Invoices & Line Items (Acme Corp)
INSERT INTO invoices (id, tenant_id, invoice_number, status, subtotal, tax_amount, total_amount, currency, due_date, issued_at, paid_at, payment_method, notes)
VALUES
  ('inv_acme_01', 'tenant_acme_corp', 'INV-2026-0081', 'Paid', 68000.00, 10200.00, 78200.00, 'SAR', NOW() - INTERVAL '30 days', NOW() - INTERVAL '45 days', NOW() - INTERVAL '40 days', 'Bank Wire Transfer', 'Invoice paid in full via corporate wire transfer.'),
  ('inv_acme_02', 'tenant_acme_corp', 'INV-2026-0094', 'Issued', 28000.00, 4200.00, 32200.00, 'SAR', NOW() + INTERVAL '14 days', NOW() - INTERVAL '5 days', NULL, 'Bank Wire Transfer', 'Payment due within 14 days.')
ON CONFLICT (id) DO NOTHING;

INSERT INTO invoice_items (id, invoice_id, description, quantity, unit_price, total_amount)
VALUES
  ('item_01', 'inv_acme_01', 'Enterprise Web Platform - Flagship Architecture Tier', 1, 68000.00, 68000.00),
  ('item_02', 'inv_acme_02', 'Cloud DevOps Architecture & Automated CI/CD Infrastructure', 1, 28000.00, 28000.00)
ON CONFLICT (id) DO NOTHING;

-- 13. Notifications (Acme Corp)
INSERT INTO notifications (id, tenant_id, recipient_user_id, title, title_ar, body, body_ar, category, link_url, is_read)
VALUES
  ('notif_acme_01', 'tenant_acme_corp', 'usr_client_001', 'Milestone 2 Completed', 'اكتمال المرحلة الثانية بنجاح', 'UI/UX interactive component implementation has been verified and signed off.', 'تم التحقق من تنفيذ مكونات واجهة المستخدم التفاعلية واعتمادها.', 'Project', '/projects', FALSE),
  ('notif_acme_02', 'tenant_acme_corp', 'usr_client_001', 'Change Request Quoted', 'تم تسعير طلب التعديل', 'Your request for Biometric MFA Verification has been estimated.', 'تم تقدير تكلفة وزمن تنفيذ طلب التحقق الثنائي البيومتري.', 'Service', '/requests', FALSE)
ON CONFLICT (id) DO NOTHING;

-- 14. Audit Logs
INSERT INTO audit_logs (id, tenant_id, actor_id, actor_name, actor_role, action, resource_type, resource_id, details)
VALUES
  ('aud_01', 'tenant_acme_corp', 'usr_admin_001', 'Platform Admin', 'Super Admin', 'SERVICE_PURCHASE_VERIFIED', 'ClientService', 'cs_acme_01', 'Activated Flagship Web Platform service for Acme Corporation.'),
  ('aud_02', 'tenant_acme_corp', 'usr_admin_001', 'Platform Admin', 'Super Admin', 'INVOICE_PAYMENT_CONFIRMED', 'Invoice', 'inv_acme_01', 'Confirmed wire transfer receipt of 78,200.00 SAR.'),
  ('aud_03', 'tenant_acme_corp', 'usr_client_001', 'Sarah Jenkins', 'Owner', 'CHANGE_REQUEST_SUBMITTED', 'ChangeRequest', 'cr_acme_01', 'Submitted scope change request for Biometric MFA.')
ON CONFLICT (id) DO NOTHING;

-- 15. Client B (Vertex Systems) - Completely Isolated Records for Tenant Boundary Testing
INSERT INTO client_services (id, tenant_id, service_id, package_name, status, price, currency, sla_tier, assigned_lead)
VALUES
  ('cs_vertex_01', 'tenant_vertex_sys', 'srv_ai_data', 'AI Big Data Pipeline', 'Active', 55000.00, 'SAR', 'Standard SLA', 'Dr. Tariq (AI Research Lead)')
ON CONFLICT (id) DO NOTHING;

INSERT INTO projects (id, tenant_id, client_service_id, title, description, status, progress_pct, priority, deadline, assigned_team)
VALUES
  ('prj_vertex_01', 'tenant_vertex_sys', 'cs_vertex_01', 'Vertex Predictive Fraud Engine', 'Private real-time fraud detection engine for Vertex Systems banking transactions.', 'In Progress', 40, 'Urgent', NOW() + INTERVAL '45 days', 'AI & Machine Learning Squad')
ON CONFLICT (id) DO NOTHING;

INSERT INTO invoices (id, tenant_id, invoice_number, status, subtotal, tax_amount, total_amount, currency, due_date, issued_at)
VALUES
  ('inv_vertex_01', 'tenant_vertex_sys', 'INV-2026-0099', 'Issued', 55000.00, 8250.00, 63250.00, 'SAR', NOW() + INTERVAL '30 days', NOW() - INTERVAL '2 days')
ON CONFLICT (id) DO NOTHING;

INSERT INTO invoice_items (id, invoice_id, description, quantity, unit_price, total_amount)
VALUES
  ('item_v01', 'inv_vertex_01', 'AI Predictive Fraud Engine - Custom Model Pipeline', 1, 55000.00, 55000.00)
ON CONFLICT (id) DO NOTHING;
