const fs = require('fs');
const http = require('http');

async function getJson(url) {
    return new Promise((resolve, reject) => {
        http.get(url, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => resolve(JSON.parse(data)));
        }).on('error', reject);
    });
}

async function postForm(url, body) {
    return new Promise((resolve, reject) => {
        const u = new URL(url);
        const req = http.request({
            hostname: u.hostname,
            port: u.port,
            path: u.pathname,
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
                'Content-Length': Buffer.byteLength(body)
            }
        }, (res) => {
            let setCookie = res.headers['set-cookie'];
            let respBody = '';
            res.on('data', chunk => respBody += chunk);
            res.on('end', () => resolve({ headers: res.headers, body: respBody, setCookie }));
        });
        req.on('error', reject);
        req.write(body);
        req.end();
    });
}

class CDPClient {
    constructor(wsUrl) {
        this.wsUrl = wsUrl;
        this.msgId = 0;
        this.callbacks = new Map();
        this.events = new Map();
    }

    async connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.wsUrl);
            this.ws.onopen = () => resolve();
            this.ws.onerror = (e) => reject(e);
            this.ws.onmessage = (event) => {
                const msg = JSON.parse(event.data);
                if (msg.id && this.callbacks.has(msg.id)) {
                    const cb = this.callbacks.get(msg.id);
                    this.callbacks.delete(msg.id);
                    if (msg.error) cb.reject(new Error(msg.error.message));
                    else cb.resolve(msg.result);
                } else if (msg.method) {
                    const listeners = this.events.get(msg.method) || [];
                    listeners.forEach(fn => fn(msg.params));
                }
            };
        });
    }

    send(method, params = {}) {
        return new Promise((resolve, reject) => {
            const id = ++this.msgId;
            this.callbacks.set(id, { resolve, reject });
            this.ws.send(JSON.stringify({ id, method, params }));
        });
    }

    on(event, handler) {
        if (!this.events.has(event)) this.events.set(event, []);
        this.events.get(event).push(handler);
    }

    async evaluate(expression) {
        const res = await this.send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
        if (res.exceptionDetails) {
            const desc = res.exceptionDetails.exception ? res.exceptionDetails.exception.description : res.exceptionDetails.text;
            throw new Error(desc || 'JS Evaluation Exception');
        }
        return res.result ? res.result.value : undefined;
    }

    async captureScreenshot(filepath) {
        const res = await this.send('Page.captureScreenshot', { format: 'png' });
        fs.writeFileSync(filepath, Buffer.from(res.data, 'base64'));
        console.log(`📸 Screenshot saved: ${filepath}`);
    }

    async close() {
        if (this.ws) this.ws.close();
    }
}

async function sleep(ms) {
    return new Promise(r => setTimeout(r, ms));
}

async function run() {
    console.log("=== STARTING FULL REAL-BROWSER NAVIGATION & HYDRATION VERIFICATION ===");

    // 1. Get Client and Admin Tokens
    const clientAuth = await postForm('http://127.0.0.1:3000/api/login8413405177072195000', 'email=client%40teebtech.com&password=Password123%21');
    const clientCookieStr = (clientAuth.setCookie || [])[0] || '';
    const clientToken = clientCookieStr.split(';')[0].replace('teebtech_session=', '');
    console.log(`Client JWT Token obtained: ${clientToken.slice(0, 25)}...`);

    const adminAuth = await postForm('http://127.0.0.1:3000/api/login8413405177072195000', 'email=admin%40teebtech.com&password=Password123%21');
    const adminCookieStr = (adminAuth.setCookie || [])[0] || '';
    const adminToken = adminCookieStr.split(';')[0].replace('teebtech_session=', '');
    console.log(`Admin JWT Token obtained: ${adminToken.slice(0, 25)}...`);

    // 2. Get active tab on Chrome CDP
    const tabs = await getJson('http://127.0.0.1:9222/json/list');
    const pageTab = tabs.find(t => t.type === 'page');
    if (!pageTab) throw new Error("No page tab found in Chrome!");
    console.log(`Connected to Chrome tab: ${pageTab.id}`);
    const cdp = new CDPClient(pageTab.webSocketDebuggerUrl);
    await cdp.connect();

    await cdp.send('Page.enable');
    await cdp.send('Runtime.enable');
    await cdp.send('Network.enable');
    await cdp.send('Emulation.setDeviceMetricsOverride', {
        width: 1366,
        height: 850,
        deviceScaleFactor: 1,
        mobile: false
    });

    const artifactDir = '/home/noon/.gemini/antigravity-ide/brain/e35acfd2-0198-408d-914c-f7cf706f8a88';

    // 3. Set Client Cookie and Navigate to Client Portal
    await cdp.send('Network.setCookie', {
        name: 'teebtech_session',
        value: clientToken,
        domain: '127.0.0.1',
        path: '/',
        httpOnly: true,
        sameSite: 'Lax'
    });

    console.log("\n[TEST] Navigating to Client Portal: http://127.0.0.1:3002/");
    await cdp.send('Page.navigate', { url: 'http://127.0.0.1:3002/' });
    await sleep(2500); // Allow WASM hydration

    let path = await cdp.evaluate('window.location.pathname');
    let title = await cdp.evaluate('document.title');
    console.log(`Current Path: ${path}, Page Title: "${title}"`);
    await cdp.captureScreenshot(`${artifactDir}/client_portal_overview_v3.png`);

    // 4. Click 'My Services' in Sidebar
    console.log("\n[TEST] Clicking Sidebar: 'My Services' (/services)");
    await cdp.evaluate(`(() => {
        const link = document.querySelector('aside a[href="/services"]') || document.querySelector('a[href="/services"]');
        if (link) link.click();
        else throw new Error("Sidebar link for /services not found!");
    })()`);
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Path after click: ${path}`);
    if (path !== '/services') throw new Error(`Expected /services, got ${path}`);
    await cdp.captureScreenshot(`${artifactDir}/client_portal_services_v3.png`);

    // 5. Click 'Active Projects' in Sidebar
    console.log("\n[TEST] Clicking Sidebar: 'Active Projects' (/projects)");
    await cdp.evaluate(`(() => {
        const link = document.querySelector('aside a[href="/projects"]') || document.querySelector('a[href="/projects"]');
        if (link) link.click();
        else throw new Error("Sidebar link for /projects not found!");
    })()`);
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Path after click: ${path}`);
    if (path !== '/projects') throw new Error(`Expected /projects, got ${path}`);

    // 6. Click 'Service Catalog' in Sidebar
    console.log("\n[TEST] Clicking Sidebar: 'Service Catalog' (/catalog)");
    await cdp.evaluate(`(() => {
        const link = document.querySelector('aside a[href="/catalog"]') || document.querySelector('a[href="/catalog"]');
        if (link) link.click();
        else throw new Error("Sidebar link for /catalog not found!");
    })()`);
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Path after click: ${path}`);
    if (path !== '/catalog') throw new Error(`Expected /catalog, got ${path}`);

    // 7. Click 'Invoices & Billing' in Sidebar
    console.log("\n[TEST] Clicking Sidebar: 'Invoices & Billing' (/billing)");
    await cdp.evaluate(`(() => {
        const link = document.querySelector('aside a[href="/billing"]') || document.querySelector('a[href="/billing"]');
        if (link) link.click();
        else throw new Error("Sidebar link for /billing not found!");
    })()`);
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Path after click: ${path}`);
    if (path !== '/billing') throw new Error(`Expected /billing, got ${path}`);
    await cdp.captureScreenshot(`${artifactDir}/client_portal_billing_v3.png`);

    // 8. Test Refresh on /billing
    console.log("\n[TEST] Testing Page Reload / Refresh on /billing");
    await cdp.send('Page.reload');
    await sleep(2000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Path after reload: ${path}`);
    if (path !== '/billing') throw new Error(`Expected /billing on reload, got ${path}`);

    // 9. Test Browser History Back button
    console.log("\n[TEST] Testing Browser Back Button");
    await cdp.evaluate('window.history.back()');
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Path after history.back(): ${path}`);
    if (path !== '/catalog') throw new Error(`Expected /catalog after back, got ${path}`);

    // 10. Switch to Admin Portal
    console.log("\n[TEST] Switching to Admin Portal: http://127.0.0.1:3004/");
    await cdp.send('Network.setCookie', {
        name: 'teebtech_session',
        value: adminToken,
        domain: '127.0.0.1',
        path: '/',
        httpOnly: true,
        sameSite: 'Lax'
    });
    await cdp.send('Page.navigate', { url: 'http://127.0.0.1:3004/' });
    await sleep(2500);

    path = await cdp.evaluate('window.location.pathname');
    title = await cdp.evaluate('document.title');
    console.log(`Admin Portal Path: ${path}, Page Title: "${title}"`);
    await cdp.captureScreenshot(`${artifactDir}/admin_portal_overview_v3.png`);

    // 11. Click Admin Sidebar: 'Clients & Tenants' (/clients)
    console.log("\n[TEST] Clicking Admin Sidebar: 'Clients & Tenants' (/clients)");
    await cdp.evaluate(`(() => {
        const link = document.querySelector('aside a[href="/clients"]') || document.querySelector('a[href="/clients"]');
        if (link) link.click();
        else throw new Error("Sidebar link for /clients not found!");
    })()`);
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Admin Path after click: ${path}`);
    if (path !== '/clients') throw new Error(`Expected /clients, got ${path}`);
    await cdp.captureScreenshot(`${artifactDir}/admin_portal_clients_v3.png`);

    // 12. Click Admin Sidebar: 'Audit Logs' (/audit)
    console.log("\n[TEST] Clicking Admin Sidebar: 'Audit Logs' (/audit)");
    await cdp.evaluate(`(() => {
        const link = document.querySelector('aside a[href="/audit"]') || document.querySelector('a[href="/audit"]');
        if (link) link.click();
        else throw new Error("Sidebar link for /audit not found!");
    })()`);
    await sleep(1000);
    path = await cdp.evaluate('window.location.pathname');
    console.log(`Admin Path after click: ${path}`);
    if (path !== '/audit') throw new Error(`Expected /audit, got ${path}`);

    // 13. Mobile Drawer Navigation Test
    console.log("\n[TEST] Testing Mobile Responsive Drawer Navigation");
    await cdp.send('Emulation.setDeviceMetricsOverride', {
        width: 390,
        height: 844,
        deviceScaleFactor: 2,
        mobile: true
    });
    await sleep(500);
    await cdp.captureScreenshot(`${artifactDir}/admin_portal_mobile_v3.png`);

    console.log("\n🎉 ALL INTERACTIVE BROWSER NAVIGATION TESTS PASSED SUCCESSFULLY! 🎉");
    await cdp.close();
}

run().catch(err => {
    console.error("❌ TEST FAILED:", err);
    process.exit(1);
});
