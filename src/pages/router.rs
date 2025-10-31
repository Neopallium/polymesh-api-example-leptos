use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::Nav;
use crate::providers::backend::use_backend_state;

use crate::pages::{AccountsPage, Connecting, DartTest, Explorer, PageNotFound, Settings};

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
                            <Route path=path!("/polymesh-api-example-leptos/") view=Explorer />
                            <Route path=path!("/polymesh-api-example-leptos/accounts") view=AccountsPage />
                            <Route path=path!("/polymesh-api-example-leptos/settings") view=Settings />
                            <Route path=path!("/polymesh-api-example-leptos/dart_test") view=DartTest />
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
