use application::TenantDto;
use leptos::prelude::*;
use ui::{Alert, Badge, Card};

#[component]
pub fn TenantDirectory(
    tenants_resource: Resource<Result<Vec<TenantDto>, ServerFnError>>,
    selected_tenant: RwSignal<Option<TenantDto>>,
) -> impl IntoView {
    view! {
        <Card
            title="Organization Directory".to_string()
            subtitle="Select an organization to inspect members and workspace permissions.".to_string()
        >
            <Suspense fallback=|| view! { <p class="text-xs text-slate-500">"Loading organizations..."</p> }>
                {move || {
                    tenants_resource.get().map(|res| match res {
                        Ok(tenants) if tenants.is_empty() => view! {
                            <p class="text-xs text-slate-400 py-4 text-center">"No organizations registered yet."</p>
                        }.into_any(),
                        Ok(tenants) => view! {
                            <div class="border border-slate-200 rounded-md divide-y divide-slate-100 overflow-hidden">
                                {tenants.into_iter().map(|tenant| {
                                    let t_clone = tenant.clone();
                                    let is_selected = move || selected_tenant.get().map(|s| s.id == t_clone.id).unwrap_or(false);
                                    let t_for_click = tenant.clone();
                                    view! {
                                        <div
                                            on:click=move |_| selected_tenant.set(Some(t_for_click.clone()))
                                            class=move || format!(
                                                "p-3 flex items-center justify-between cursor-pointer transition-colors {}",
                                                if is_selected() { "bg-brand-50 border-l-4 border-brand-600" } else { "hover:bg-slate-50" }
                                            )
                                        >
                                            <div>
                                                <p class="text-xs font-semibold text-slate-900">{tenant.name}</p>
                                                <p class="text-[10px] text-slate-400 font-mono">"slug: " {tenant.slug}</p>
                                            </div>
                                            <Badge variant="success".to_string()>
                                                "Active"
                                            </Badge>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <Alert variant="error".to_string()>{e.to_string()}</Alert>
                        }.into_any(),
                    })
                }}
            </Suspense>
        </Card>
    }
}
