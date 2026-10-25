use crate::server::authentication::{get_current_user, logout};
use application::CurrentUserDto;
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct SessionContext {
    pub current_user: Resource<Result<Option<CurrentUserDto>, ServerFnError>>,
    pub logout_action: Action<(), Result<(), ServerFnError>>,
}

impl SessionContext {
    pub fn new() -> Self {
        let current_user = Resource::new(|| (), |_| async move { get_current_user().await });
        let logout_action = Action::new(move |_: &()| async move { logout().await });

        let ctx = Self {
            current_user,
            logout_action,
        };

        Effect::new(move |_| {
            if let Some(res) = ctx.logout_action.value().get() {
                if res.is_ok() {
                    ctx.current_user.refetch();
                }
            }
        });

        ctx
    }

    pub fn refetch(&self) {
        self.current_user.refetch();
    }

    pub fn logout(&self) {
        self.logout_action.dispatch(());
    }
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new()
    }
}
