use leptos::prelude::*;

use serde::{Deserialize, Serialize};

use codee::string::JsonSerdeCodec;
use leptos_use::storage::use_local_storage;

use crate::providers::backend::*;

const APP_KEY: &str = "example.app.polymesh.network";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub url: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            url: "wss://testnet-rpc.polymesh.live/".into(),
        }
    }
}

impl AppSettings {
    pub fn update_settings(&mut self, settings: Self) {
        *self = settings;
        log::info!("app settings = {:#?}", self);
    }
}

#[component]
pub fn SettingsProvider(children: Children) -> impl IntoView {
    let (stored_settings, set_stored_settings, _) =
        use_local_storage::<AppSettings, JsonSerdeCodec>(APP_KEY);

    let (backend, set_backend) = use_backend();
    let (_, set_state) = use_backend_state();

    // Get the URL from storage, or use default
    let settings = stored_settings.get_untracked();
    let url = settings.url.clone();

    // Set backend URL on mount - only run once
    Effect::new(move |_| {
        let mut b = backend.get_untracked();
        log::info!("Connecting to backend at URL: {}", url);
        b.connect_to(url.clone(), set_state);
        set_backend.set(b);
    });

    provide_context(stored_settings);
    provide_context(set_stored_settings);

    children()
}

pub fn use_settings() -> (ReadSignal<AppSettings>, WriteSignal<AppSettings>) {
    (
        use_context::<ReadSignal<AppSettings>>().expect("Settings context"),
        use_context::<WriteSignal<AppSettings>>().expect("Settings setter context"),
    )
}
