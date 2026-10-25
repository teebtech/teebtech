use crate::server::members::invite_member;
use application::{MemberDto, TenantDto};
use leptos::prelude::*;
use ui::{Alert, Button, Card};

#[component]
pub fn InviteMemberForm(
    selected_tenant: RwSignal<Option<TenantDto>>,
    #[prop(into)] on_member_invited: Callback<MemberDto>,
) -> impl IntoView {
    let invite_action = Action::new(
        move |(tid, email, name, role): &(String, String, String, String)| {
            let tid = tid.clone();
            let email = email.clone();
            let name = name.clone();
            let role = role.clone();
            async move { invite_member(tid, email, name, role).await }
        },
    );

    let member_email_signal = RwSignal::new(String::new());
    let member_name_signal = RwSignal::new(String::new());
    let member_role_signal = RwSignal::new("Member".to_string());
    let member_error_signal = RwSignal::new(Option::<String>::None);
    let member_success_signal = RwSignal::new(Option::<String>::None);

    let on_invited = on_member_invited;

    Effect::new(move |_| {
        if let Some(res) = invite_action.value().get() {
            match res {
                Ok(member) => {
                    member_error_signal.set(None);
                    member_success_signal.set(Some(format!(
                        "Member '{}' ({}) successfully invited!",
                        member.name, member.email
                    )));
                    member_email_signal.set(String::new());
                    member_name_signal.set(String::new());
                    member_role_signal.set("Member".to_string());
                    on_invited.run(member);
                }
                Err(err) => {
                    member_success_signal.set(None);
                    member_error_signal.set(Some(err.to_string()));
                }
            }
        }
    });

    let on_member_email_input = move |ev: leptos::ev::Event| {
        member_email_signal.set(event_target_value(&ev));
    };

    let on_member_name_input = move |ev: leptos::ev::Event| {
        member_name_signal.set(event_target_value(&ev));
    };

    let on_member_role_change = move |ev: leptos::ev::Event| {
        member_role_signal.set(event_target_value(&ev));
    };

    let on_member_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let active_tenant = selected_tenant.get();
        if active_tenant.is_none() {
            member_error_signal.set(Some(
                "Please select an organization from the directory below first.".to_string(),
            ));
            return;
        }

        let tid = active_tenant.unwrap().id;
        let email = member_email_signal.get();
        let name = member_name_signal.get();
        let role = member_role_signal.get();

        if email.trim().is_empty() || name.trim().is_empty() {
            member_error_signal.set(Some("Email and Name are required.".to_string()));
            return;
        }

        invite_action.dispatch((tid, email, name, role));
    };

    view! {
        <Card
            title="Invite Team Member".to_string()
            subtitle="Add authenticated members to the selected active organization.".to_string()
        >
            {move || member_error_signal.get().map(|err| view! {
                <Alert variant="error".to_string() class="mb-4".to_string()>
                    {err}
                </Alert>
            })}

            {move || member_success_signal.get().map(|msg| view! {
                <Alert variant="success".to_string() class="mb-4".to_string()>
                    {msg}
                </Alert>
            })}

            <form on:submit=on_member_submit class="space-y-4">
                <div class="space-y-1">
                    <label class="text-xs font-medium text-slate-700">"Member Full Name"</label>
                    <input
                        type="text"
                        placeholder="Jane Doe"
                        prop:value=move || member_name_signal.get()
                        on:input=on_member_name_input
                        class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                    />
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <div class="space-y-1">
                        <label class="text-xs font-medium text-slate-700">"Email Address"</label>
                        <input
                            type="email"
                            placeholder="jane@company.com"
                            prop:value=move || member_email_signal.get()
                            on:input=on_member_email_input
                            class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                        />
                    </div>

                    <div class="space-y-1">
                        <label class="text-xs font-medium text-slate-700">"Workspace Role"</label>
                        <select
                            on:change=on_member_role_change
                            class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                        >
                            <option value="Member">"Member"</option>
                            <option value="Admin">"Admin"</option>
                            <option value="Owner">"Owner"</option>
                        </select>
                    </div>
                </div>

                <Button loading=invite_action.pending().get()>
                    {move || if invite_action.pending().get() { "Inviting..." } else { "Send Invitation" }}
                </Button>
            </form>
        </Card>
    }
}
