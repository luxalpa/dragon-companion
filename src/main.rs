#[cfg(feature = "ssr")]
fn main() -> anyhow::Result<()> {
    companion::server_main::server_main()
}

#[cfg(feature = "ssr")]
#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos_test::app::*;

    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
