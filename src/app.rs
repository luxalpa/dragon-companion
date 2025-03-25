use leptos::prelude::*;
use leptos_animate::{AnimatedSwap, FadeAnimation, LayoutEntry, SizeTransition};
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{StaticSegment, WildcardSegment};
use leptos_use::{ThrottleOptions, use_throttle_fn_with_options};
use std::time::Duration;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/companion.css"/>

        // sets the document title
        <Title text="Companion"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

// const COMPLETE_ICON_SVG: &'static str = include_str!("../assets/process-completed.svg");

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let tmp_switch = RwSignal::new(false);

    let throttle_fn = use_throttle_fn_with_options(
        move || tmp_switch.update(|v| *v = !*v),
        1000.0,
        ThrottleOptions::default().trailing(false),
    );

    let cur_task_fn = move || {
        if tmp_switch.get() {
            LayoutEntry {
                key: 1,
                view_fn: Box::new(move || {
                    view! {
                        <div>"Other task with a very long name"<br/>"Over multiple lines!"</div>
                    }
                    .into_any()
                }),
            }
        } else {
            LayoutEntry {
                key: 2,
                view_fn: Box::new(move || {
                    view! {
                        <div>"Clean the kitchen"</div>
                    }
                    .into_any()
                }),
            }
        }
    };

    let anim = FadeAnimation::new(Duration::from_millis(200), "ease-out");

    view! {
        <div class="main-grid">
            <div class="next-task-label">
                "Next task:"
            </div>
            <div class="current-task" on:click=move |_| { throttle_fn(); }>
                <SizeTransition>
                    <AnimatedSwap contents=cur_task_fn enter_anim=anim.clone() leave_anim=anim />
                </SizeTransition>
                // <div class="complete-icon" inner_html=COMPLETE_ICON_SVG></div>
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
