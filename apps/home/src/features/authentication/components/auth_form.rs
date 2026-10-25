use crate::server::authentication::{login, register_user};
use leptos::prelude::*;
use ui::{Alert, Button};

#[component]
pub fn AuthForm(#[prop(into)] on_auth_success: Callback<()>) -> impl IntoView {
    let auth_mode_signal = RwSignal::new("login".to_string()); // "login" or "register"

    let login_action = Action::new(move |(email, password): &(String, String)| {
        let email = email.clone();
        let password = password.clone();
        async move { login(email, password).await }
    });

    let register_user_action =
        Action::new(move |(email, password, name): &(String, String, String)| {
            let email = email.clone();
            let password = password.clone();
            let name = name.clone();
            async move { register_user(email, password, name).await }
        });

    let auth_email_signal = RwSignal::new(String::new());
    let auth_password_signal = RwSignal::new(String::new());
    let auth_name_signal = RwSignal::new(String::new());
    let auth_error_signal = RwSignal::new(Option::<String>::None);
    let auth_success_signal = RwSignal::new(Option::<String>::None);

    let on_success_login = on_auth_success;
    let on_success_reg = on_auth_success;

    Effect::new(move |_| {
        if let Some(res) = login_action.value().get() {
            match res {
                Ok(user) => {
                    auth_error_signal.set(None);
                    auth_success_signal.set(Some(format!(
                        "Welcome back, {}! Session established.",
                        user.display_name
                    )));
                    auth_password_signal.set(String::new());
                    on_success_login.run(());
                }
                Err(err) => {
                    auth_success_signal.set(None);
                    auth_error_signal.set(Some(err.to_string()));
                }
            }
        }
    });

    Effect::new(move |_| {
        if let Some(res) = register_user_action.value().get() {
            match res {
                Ok(user) => {
                    auth_error_signal.set(None);
                    auth_success_signal.set(Some(format!(
                        "Account created for {}! Session active.",
                        user.display_name
                    )));
                    auth_password_signal.set(String::new());
                    on_success_reg.run(());
                }
                Err(err) => {
                    auth_success_signal.set(None);
                    auth_error_signal.set(Some(err.to_string()));
                }
            }
        }
    });

    let on_auth_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let mode = auth_mode_signal.get();
        let email = auth_email_signal.get();
        let password = auth_password_signal.get();

        if email.trim().is_empty() || password.trim().is_empty() {
            auth_error_signal.set(Some("Email and password are required.".to_string()));
            return;
        }

        if mode == "register" {
            let name = auth_name_signal.get();
            if name.trim().is_empty() {
                auth_error_signal.set(Some(
                    "Display name is required for registration.".to_string(),
                ));
                return;
            }
            register_user_action.dispatch((email, password, name));
        } else {
            login_action.dispatch((email, password));
        }
    };

    view! {
        <div id="auth-section" class="bg-white border border-slate-200 rounded-lg p-6 shadow-sm">
            <div class="flex flex-col md:flex-row md:items-center justify-between border-b border-slate-100 pb-4 mb-6 gap-4">
                <div>
                    <h2 class="font-display text-xl font-semibold text-slate-900">
                        "User Identity & Secure Authentication"
                    </h2>
                    <p class="text-xs text-slate-500 mt-0.5">
                        "Production-hardened HttpOnly session management with zero raw token leakage to client-side state."
                    </p>
                </div>
                <div class="flex items-center bg-slate-100 p-1 rounded-lg border border-slate-200">
                    <button
                        on:click=move |_| auth_mode_signal.set("login".to_string())
                        class=move || format!(
                            "px-3 py-1.5 text-xs rounded-md font-medium transition-all cursor-pointer {}",
                            if auth_mode_signal.get() == "login" {
                                "bg-white text-slate-900 shadow-sm border border-slate-200"
                            } else {
                                "text-slate-600 hover:text-slate-900"
                            }
                        )
                    >
                        "Sign In"
                    </button>
                    <button
                        on:click=move |_| auth_mode_signal.set("register".to_string())
                        class=move || format!(
                            "px-3 py-1.5 text-xs rounded-md font-medium transition-all cursor-pointer {}",
                            if auth_mode_signal.get() == "register" {
                                "bg-white text-slate-900 shadow-sm border border-slate-200"
                            } else {
                                "text-slate-600 hover:text-slate-900"
                            }
                        )
                    >
                        "Register Account"
                    </button>
                </div>
            </div>

            {move || auth_error_signal.get().map(|err| view! {
                <Alert variant="error".to_string() class="mb-4".to_string()>
                    {err}
                </Alert>
            })}

            {move || auth_success_signal.get().map(|msg| view! {
                <Alert variant="success".to_string() class="mb-4".to_string()>
                    {msg}
                </Alert>
            })}

            <form on:submit=on_auth_submit class="space-y-4 max-w-xl">
                {move || if auth_mode_signal.get() == "register" {
                    view! {
                        <div class="space-y-1">
                            <label class="text-xs font-medium text-slate-700">"Full Name"</label>
                            <input
                                type="text"
                                placeholder="Alice Engineer"
                                prop:value=move || auth_name_signal.get()
                                on:input=move |ev| auth_name_signal.set(event_target_value(&ev))
                                class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                            />
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="hidden"></div> }.into_any()
                }}

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="space-y-1">
                        <label class="text-xs font-medium text-slate-700">"Email Address"</label>
                        <input
                            type="email"
                            placeholder="alice@example.com"
                            prop:value=move || auth_email_signal.get()
                            on:input=move |ev| auth_email_signal.set(event_target_value(&ev))
                            class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                        />
                    </div>

                    <div class="space-y-1">
                        <label class="text-xs font-medium text-slate-700">"Password"</label>
                        <input
                            type="password"
                            placeholder="••••••••"
                            prop:value=move || auth_password_signal.get()
                            on:input=move |ev| auth_password_signal.set(event_target_value(&ev))
                            class="w-full h-9 px-3 text-sm bg-white border border-slate-300 rounded-md text-slate-900 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-brand-500 transition-shadow"
                        />
                    </div>
                </div>

                <div class="pt-2">
                    <Button
                        loading=login_action.pending().get() || register_user_action.pending().get()
                        size="md".to_string()
                    >
                        {move || if auth_mode_signal.get() == "register" {
                            if register_user_action.pending().get() { "Creating Account..." } else { "Create Account" }
                        } else {
                            if login_action.pending().get() { "Authenticating..." } else { "Sign In" }
                        }}
                    </Button>
                </div>
            </form>
        </div>
    }
}
