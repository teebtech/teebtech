use application::MemberDto;
use leptos::prelude::*;
use ui::{Alert, Badge, Card};

#[component]
pub fn MemberRoster(
    members_resource: Resource<Result<Vec<MemberDto>, ServerFnError>>,
) -> impl IntoView {
    view! {
        <Card
            title="Active Team Roster".to_string()
            subtitle="Members with access to the currently selected organization.".to_string()
        >
            <Suspense fallback=|| view! { <p class="text-xs text-slate-500">"Loading team members..."</p> }>
                {move || {
                    members_resource.get().map(|res| match res {
                        Ok(members) if members.is_empty() => view! {
                            <p class="text-xs text-slate-400 py-4 text-center">"Select an organization above to view members."</p>
                        }.into_any(),
                        Ok(members) => view! {
                            <div class="border border-slate-200 rounded-md divide-y divide-slate-100 overflow-hidden">
                                {members.into_iter().map(|m| view! {
                                    <div class="p-3 flex items-center justify-between">
                                        <div>
                                            <p class="text-xs font-semibold text-slate-900">{m.name}</p>
                                            <p class="text-[10px] text-slate-400 font-mono">{m.email}</p>
                                        </div>
                                        <Badge variant="navy".to_string()>
                                            {m.role}
                                        </Badge>
                                    </div>
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
