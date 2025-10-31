use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::components::Nav;
use crate::providers::backend::use_backend_state;

use crate::pages::{AccountsPage, Connecting, Explorer, PageNotFound, Settings};

#[component]
pub fn PageRouter() -> impl IntoView {
    let (backend_state, _) = use_backend_state();

    view! {
        <Router>
            <Nav />
            <main>
            {move || {
                if backend_state.get().is_connected() {
                    Either::Left(view! {
                        <Routes fallback=PageNotFound>
                            <Route path=StaticSegment("") view=Explorer />
                            <Route path=StaticSegment("accounts") view=AccountsPage />
                            <Route path=StaticSegment("settings") view=Settings />
                        </Routes>
                    })
                } else {
                    Either::Right(view! { <Connecting /> })
                }
            }}
            </main>
        </Router>
    }
}
