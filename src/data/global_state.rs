use leptos::{create_rw_signal, RwSignal, SignalGet, SignalGetUntracked, SignalSet};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::Storage;

#[derive(Clone, Debug, Default)]
pub(crate) struct GlobalState {
    pub _user_token: RwSignal<Option<String>>,
    pub user_name: RwSignal<Option<String>>,
    pub first_name: RwSignal<Option<String>>,
    pub last_name: RwSignal<Option<String>>,
    pub id: RwSignal<Option<i32>>,
    pub role: RwSignal<Option<String>>,
    pub authenticated: RwSignal<bool>,
}

impl GlobalState {
    pub fn new() -> Self {
        let state = Self {
            _user_token: create_rw_signal(None),
            user_name: create_rw_signal(None),
            first_name: create_rw_signal(None),
            last_name: create_rw_signal(None),
            id: create_rw_signal(None),
            role: create_rw_signal(None),
            authenticated: create_rw_signal(false),
        };

        #[cfg(target_arch = "wasm32")]
        state.load_from_local_storage();

        state
    }

    // Method to get localStorage in WASM targets
    #[cfg(target_arch = "wasm32")]
    fn local_storage() -> Storage {
        web_sys::window()
            .expect("should have a window in this context")
            .local_storage()
            .expect("should have access to localStorage")
            .expect("could not get localStorage")
    }

    // Load user data from localStorage on app initialization (WASM only)
    #[cfg(target_arch = "wasm32")]
    pub fn load_from_local_storage(&self) {
        let storage = Self::local_storage();

        if let Some(username) = storage.get_item("user_name").unwrap_or(None) {
            self.user_name.set(Some(username));
        }
        if let Some(first_name) = storage.get_item("first_name").unwrap_or(None) {
            self.first_name.set(Some(first_name));
        }
        if let Some(last_name) = storage.get_item("last_name").unwrap_or(None) {
            self.last_name.set(Some(last_name));
        }
        if let Some(id) = storage.get_item("id").unwrap_or(None) {
            self.id.set(Some(id.parse().unwrap()));
        }
        if let Some(role) = storage.get_item("role").unwrap_or(None) {
            self.role.set(Some(role));
        }
        if self.user_name.get_untracked().is_some() {
            self.authenticated.set(true);
        }
    }

    // No-op version for non-WASM targets
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_local_storage(&self) {
        // No operation
    }

    // Save user data to localStorage after login (WASM only)
    #[cfg(target_arch = "wasm32")]
    pub fn save_to_local_storage(&self) {
        let storage = Self::local_storage();

        if let Some(username) = self.user_name.get_untracked() {
            storage.set_item("user_name", &username).unwrap();
        }
        if let Some(first_name) = self.first_name.get_untracked() {
            storage.set_item("first_name", &first_name).unwrap();
        }
        if let Some(last_name) = self.last_name.get_untracked() {
            storage.set_item("last_name", &last_name).unwrap();
        }
        if let Some(id) = self.id.get_untracked() {
            storage.set_item("id", &id.to_string()).unwrap();
        }
        if let Some(role) = self.role.get_untracked() {
            storage.set_item("role", &role).unwrap();
        }
    }

    // No-op version for non-WASM targets
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_local_storage(&self) {
        // No operation
    }

    // Clear localStorage and log out (WASM only)
    #[cfg(target_arch = "wasm32")]
    pub fn clear_local_storage(&self) {
        let storage = Self::local_storage();

        storage.remove_item("user_name").unwrap();
        storage.remove_item("first_name").unwrap();
        storage.remove_item("last_name").unwrap();
        storage.remove_item("id").unwrap();
        storage.remove_item("role").unwrap();

        // Reset the state
        self.authenticated.set(false);
        self.user_name.set(None);
        self.first_name.set(None);
        self.last_name.set(None);
        self.id.set(None);
        self.role.set(None);
    }

    // No-op version for non-WASM targets
    #[cfg(not(target_arch = "wasm32"))]
    pub fn clear_local_storage(&self) {
        // No operation
    }
}
