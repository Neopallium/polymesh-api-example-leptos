use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (navbar_active, set_navbar_active) = signal(false);

    let toggle_navbar = move |_| {
        set_navbar_active.update(|active| *active = !*active);
    };

    view! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "Polymesh app" }</h1>

                <button
                    class=move || if navbar_active.get() { "navbar-burger burger is-active" } else { "navbar-burger burger" }
                    aria-label="menu"
                    aria-expanded="false"
                    on:click=toggle_navbar
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class=move || if navbar_active.get() { "navbar-menu is-active" } else { "navbar-menu" }>
                <div class="navbar-start">
                    <a href="/accounts" class="navbar-item">
                        { "Accounts" }
                    </a>

                    <div class="navbar-item has-dropdown is-hoverable">
                        <div class="navbar-link">
                            { "Network" }
                        </div>
                        <div class="navbar-dropdown">
                            <a href="/" class="navbar-item">
                                { "Explorer" }
                            </a>
                        </div>
                    </div>

                    <a href="/settings" class="navbar-item">
                        { "Settings" }
                    </a>
                </div>

                <div class="navbar-end">
                    <div class="navbar-item">
                    </div>
                </div>
            </div>
        </nav>
    }
}
