pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use polymesh_api_example_leptos::app::*;

    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
