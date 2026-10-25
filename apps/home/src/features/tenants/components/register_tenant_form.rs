use crate::server::tenants::register_tenant;
use application::TenantDto;
use leptos::prelude::*;
use ui::{Alert, Button, Card};

#[component]
pub fn RegisterTenantForm(#[prop(into)] on_tenant_created: Callback<TenantDto>) -> impl IntoView {
    let register_tenant_action = Action::new(move |(name, slug): &(String, String)| {
        let name = name.clone();
        let slug = slug.clone();
        async move { register_tenant(name, slug).await }
    });

    let tenant_name_signal = RwSignal::new(String::new());
    let tenant_slug_signal = RwSignal::new(String::new());
    let tenant_error_signal = RwSignal::new(Option::<String>::None);
    let tenant_success_signal = RwSignal::new(Option::<String>::None);

    let on_created = on_tenant_created;

    Effect::new(move |_| {
        if let Some(res) = register_tenant_action.value().get() {
            match res {
                Ok(tenant) => {
                    tenant_error_signal.set(None);
                    tenant_success_signal.set(Some(format!(
                        "Organization '{}' registered successfully!",
                        tenant.name
                    )));
                    tenant_name_signal.set(String::new());
                    tenant_slug_signal.set(String::new());
                    on_created.run(tenant);
                }
                Err(err) => {
                    tenant_success_signal.set(None);
                    tenant_error_signal.set(Some(err.to_string()));
                }
            }
        }
    });

    let on_tenant_name_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        tenant_name_signal.set(val.clone());

        let auto_slug = val
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect::<String>();
        tenant_slug_signal.set(auto_slug);
    };

    let on_tenant_slug_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        tenant_slug_signal.set(val);
    };

    let on_tenant_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let name = tenant_name_signal.get();
        let slug = tenant_slug_signal.get();

        if name.trim().is_empty() || slug.trim().is_empty() {
            tenant_error_signal.set(Some("Name and slug fields are required.".to_string()));
            return;
        }

        register_tenant_action.dispatch((name, slug));
    };

    view! {
        <Card
            title="Register Organization".to_string()
            subtitle="Create a new multi-tenant workspace with isolated boundaries.".to_string()
        >
            {move || tenant_error_signal.get().map(|err| view! {
                <Alert variant="error".to_string() class="mb-4".to_string()>
                    {err}
                </Alert>
            })}

            {move || tenant_success_signal.get().map(|msg| view! {
                <Alert variant="success".to_string() class="mb-4".to_string()>
                    {msg}
                </Alert>
            })}

            <form on:submit=on_tenant_submit class="space-y-4">
                <div class="space-y-1">
                    <label class="text-xs font-medium text-slate-700">"Organization Name"</label>
                    <input
                        type="text"
                        placeholder="Acme Corp"
                        prop:value=move || tenant_name_signal.get()
                        on:input=on_tenant_name_input
                        class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                    />
                </div>

                <div class="space-y-1">
                    <label class="text-xs font-medium text-slate-700">"Workspace Slug"</label>
                    <input
                        type="text"
                        placeholder="acme-corp"
                        prop:value=move || tenant_slug_signal.get()
                        on:input=on_tenant_slug_input
                        class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow font-mono text-xs"
                    />
                </div>

                <Button loading=register_tenant_action.pending().get()>
                    {move || if register_tenant_action.pending().get() { "Registering..." } else { "Register Organization" }}
                </Button>
            </form>
        </Card>
    }
}
