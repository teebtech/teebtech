#!/usr/bin/env bash
set -euo pipefail

# Complete Zero-Trust Portal Navigation & Route Protection Verification Matrix
echo "=========================================================================="
echo "TEEBTECH COMPLETE PORTAL NAVIGATION & ZERO-TRUST ROUTE VERIFICATION MATRIX"
echo "=========================================================================="

FAILED=0
PASSED=0

assert_status_and_content() {
    local test_name="$1"
    local url="$2"
    local cookie_header="$3"
    local expected_status="$4"
    local expected_content="$5"

    local status
    if [ -n "$cookie_header" ]; then
        status=$(curl -s -o /tmp/resp.txt -w "%{http_code}" -H "Cookie: $cookie_header" "$url")
    else
        status=$(curl -s -o /tmp/resp.txt -w "%{http_code}" "$url")
    fi

    if [ "$status" != "$expected_status" ]; then
        echo "❌ FAIL: $test_name"
        echo "   URL: $url"
        echo "   Expected HTTP $expected_status, got $status"
        FAILED=$((FAILED + 1))
        return
    fi

    if [ -n "$expected_content" ] && ! grep -Fq "$expected_content" /tmp/resp.txt; then
        echo "❌ FAIL: $test_name"
        echo "   URL: $url"
        echo "   Expected body to contain: '$expected_content'"
        echo "   First 200 chars of body: $(head -c 200 /tmp/resp.txt)"
        FAILED=$((FAILED + 1))
        return
    fi

    echo "✅ PASS: $test_name (HTTP $status)"
    PASSED=$((PASSED + 1))
}

echo ""
echo "--- 1. PUBLIC WEBSITE & AUTH GATEWAY (Port 3000) ---"
assert_status_and_content "Public Homepage" "http://127.0.0.1:3000/" "" "200" "TeebTech"
assert_status_and_content "Public Services Page" "http://127.0.0.1:3000/services" "" "200" "Services"
assert_status_and_content "Public Case Studies Page" "http://127.0.0.1:3000/case-studies" "" "200" "Case Studies"
assert_status_and_content "Public Contact Page" "http://127.0.0.1:3000/contact" "" "200" "Contact"
assert_status_and_content "Login Page" "http://127.0.0.1:3000/login" "" "200" "Sign in to your client or staff portal"
assert_status_and_content "Login with Return To Parameter" "http://127.0.0.1:3000/login?return_to=http%3A%2F%2F127.0.0.1%3A3002%2Fservices" "" "200" "Sign in to your client or staff portal"

echo ""
echo "--- 2. CLIENT PORTAL UNAUTHENTICATED ROUTE PROTECTION (Port 3002) ---"
assert_status_and_content "Client Unauthenticated Root" "http://127.0.0.1:3002/" "" "200" "Access Denied"
assert_status_and_content "Client Unauthenticated Services" "http://127.0.0.1:3002/services" "" "200" "return_to=http://127.0.0.1:3002/services"
assert_status_and_content "Client Unauthenticated Billing" "http://127.0.0.1:3002/billing" "" "200" "return_to=http://127.0.0.1:3002/billing"
assert_status_and_content "Client 404 Fallback View" "http://127.0.0.1:3002/unknown-subroute-test" "" "404" "Portal Page Not Found"

echo ""
echo "--- 3. CLIENT PORTAL AUTHENTICATED DIRECT ROUTE NAVIGATION (Port 3002) ---"
CLIENT_LOGIN_RESP=$(curl -s -i -X POST -H "Content-Type: application/x-www-form-urlencoded" -d "email=client%40teebtech.com&password=Password123%21" http://127.0.0.1:3000/api/login8413405177072195000)
CLIENT_COOKIE=$(echo "$CLIENT_LOGIN_RESP" | grep -i "set-cookie:" | sed -e 's/[sS]et-[cC]ookie: //i' | cut -d';' -f1 | tr -d '\r\n')
echo "Obtained Client Session: ${CLIENT_COOKIE:0:35}..."

assert_status_and_content "Client Overview (/)" "http://127.0.0.1:3002/" "$CLIENT_COOKIE" "200" "Sarah Jenkins"
assert_status_and_content "Client Overview (/overview)" "http://127.0.0.1:3002/overview" "$CLIENT_COOKIE" "200" "Sarah Jenkins"
assert_status_and_content "Client Services (/services)" "http://127.0.0.1:3002/services" "$CLIENT_COOKIE" "200" "My Subscribed Services"
assert_status_and_content "Client Projects (/projects)" "http://127.0.0.1:3002/projects" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Catalog (/catalog)" "http://127.0.0.1:3002/catalog" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Requests (/requests)" "http://127.0.0.1:3002/requests" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Messages (/messages)" "http://127.0.0.1:3002/messages" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Deliverables (/deliverables)" "http://127.0.0.1:3002/deliverables" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Files Alias (/files)" "http://127.0.0.1:3002/files" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Billing (/billing)" "http://127.0.0.1:3002/billing" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Invoices Alias (/invoices)" "http://127.0.0.1:3002/invoices" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Activity (/activity)" "http://127.0.0.1:3002/activity" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client History Alias (/history)" "http://127.0.0.1:3002/history" "$CLIENT_COOKIE" "200" "Acme Corporation"
assert_status_and_content "Client Settings (/settings)" "http://127.0.0.1:3002/settings" "$CLIENT_COOKIE" "200" "Acme Corporation"

echo ""
echo "--- 4. ADMIN PORTAL ZERO-TRUST AUTHORIZATION & ROUTE GUARDS (Port 3004) ---"
assert_status_and_content "Admin Unauthenticated Root" "http://127.0.0.1:3004/" "" "200" "Administrative Privileges Required"
assert_status_and_content "Admin Unauthenticated Clients" "http://127.0.0.1:3004/clients" "" "200" "return_to=http://127.0.0.1:3004/clients"
assert_status_and_content "Admin Client-Role Rejected" "http://127.0.0.1:3004/clients" "$CLIENT_COOKIE" "200" "strictly restricted to platform administrators"
assert_status_and_content "Admin 404 Fallback View" "http://127.0.0.1:3004/unknown-admin-route" "" "404" "Admin Route Not Found"

echo ""
echo "--- 5. ADMIN PORTAL AUTHENTICATED DIRECT ROUTE NAVIGATION (Port 3004) ---"
ADMIN_LOGIN_RESP=$(curl -s -i -X POST -H "Content-Type: application/x-www-form-urlencoded" -d "email=admin%40teebtech.com&password=Password123%21" http://127.0.0.1:3000/api/login8413405177072195000)
ADMIN_COOKIE=$(echo "$ADMIN_LOGIN_RESP" | grep -i "set-cookie:" | sed -e 's/[sS]et-[cC]ookie: //i' | cut -d';' -f1 | tr -d '\r\n')
echo "Obtained Admin Session: ${ADMIN_COOKIE:0:35}..."

assert_status_and_content "Admin Overview (/)" "http://127.0.0.1:3004/" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Overview (/overview)" "http://127.0.0.1:3004/overview" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Clients (/clients)" "http://127.0.0.1:3004/clients" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Catalog (/catalog)" "http://127.0.0.1:3004/catalog" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Services (/services)" "http://127.0.0.1:3004/services" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Projects (/projects)" "http://127.0.0.1:3004/projects" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Requests (/requests)" "http://127.0.0.1:3004/requests" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Messages (/messages)" "http://127.0.0.1:3004/messages" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Deliverables (/deliverables)" "http://127.0.0.1:3004/deliverables" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Billing (/billing)" "http://127.0.0.1:3004/billing" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Invoices Alias (/invoices)" "http://127.0.0.1:3004/invoices" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin Audit (/audit)" "http://127.0.0.1:3004/audit" "$ADMIN_COOKIE" "200" "Super Admin"
assert_status_and_content "Admin History Alias (/history)" "http://127.0.0.1:3004/history" "$ADMIN_COOKIE" "200" "Super Admin"

echo ""
echo "=========================================================================="
echo "TEST SUMMARY: $PASSED passed, $FAILED failed"
echo "=========================================================================="

if [ "$FAILED" -gt 0 ]; then
    exit 1
fi
exit 0
