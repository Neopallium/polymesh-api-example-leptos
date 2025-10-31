use leptos::prelude::*;

use leptos_meta::{provide_meta_context, Stylesheet, Title};

use crate::pages::PageRouter;
use crate::providers::accounts::AccountsProvider;
use crate::providers::backend::BackendProvider;
use crate::providers::blocks::BlocksProvider;
use crate::providers::settings::SettingsProvider;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/polymesh-api-example-leptos.css"/>

        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.0/css/bulma.min.css" />

        // sets the document title
        <Title text="Polymesh Leptos example"/>

        // content for the page
        <BackendProvider>
          <SettingsProvider>
            <AccountsProvider>
              <BlocksProvider>
                <PageRouter />
              </BlocksProvider>
            </AccountsProvider>
          </SettingsProvider>
        </BackendProvider>
    }
}
