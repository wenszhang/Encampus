use leptos::{create_rw_signal, RwSignal};

#[derive(Clone, Debug, Default)]
pub(crate) struct GlobalState {
    pub _user_token: RwSignal<Option<String>>,
    pub user_name: RwSignal<Option<String>>,
    pub first_name: RwSignal<Option<String>>,
    pub id: RwSignal<Option<i32>>,
    pub role: RwSignal<Option<String>>,
    pub authenticated: RwSignal<bool>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            _user_token: create_rw_signal(None),
            user_name: create_rw_signal(None),
            first_name: create_rw_signal(None),
            id: create_rw_signal(None),
            role: create_rw_signal(None),
            authenticated: create_rw_signal(false),
        }
    }
}
