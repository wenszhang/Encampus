use leptos::{create_rw_signal, RwSignal, SignalSet};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::Storage;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct UserState {
    pub user_token: Option<String>,
    pub user_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub id: Option<i32>,
    pub role: Option<String>,
    pub authenticated: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct GlobalState {
    pub user_state: RwSignal<UserState>,
}

impl GlobalState {
    pub fn new() -> Self {
        let state = Self {
            user_state: create_rw_signal(UserState::default()),
        };

        #[cfg(target_arch = "wasm32")]
        state.load_from_local_storage();

        state
    }

    #[cfg(target_arch = "wasm32")]
    fn local_storage() -> Option<Storage> {
        web_sys::window()?.local_storage().ok().flatten()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_from_local_storage(&self) {
        if let Some(storage) = Self::local_storage() {
            if let Ok(Some(user_data)) = storage.get_item("user_state") {
                if let Ok(user_state) = serde_json::from_str::<UserState>(&user_data) {
                    self.user_state.set(user_state);
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_local_storage(&self) {
        // No operation
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_to_local_storage(&self) {
        if let Some(storage) = Self::local_storage() {
            let user_state = self.user_state.get_untracked();
            if let Ok(user_data) = serde_json::to_string(&user_state) {
                if let Err(e) = storage.set_item("user_state", &user_data) {
                    web_sys::console::error_1(&e.into());
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_local_storage(&self) {
        // No operation
    }

    #[cfg(target_arch = "wasm32")]
    pub fn clear_local_storage(&self) {
        if let Some(storage) = Self::local_storage() {
            if let Err(e) = storage.remove_item("user_state") {
                web_sys::console::error_1(&e.into());
            }
        }

        // Reset the state
        self.user_state.set(UserState::default());
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear_local_storage(&self) {
        // No operation
    }
}