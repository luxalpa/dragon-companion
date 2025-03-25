use leptos::prelude::*;
use leptos::task::spawn_local;
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
    let cur_task = RwSignal::new("".to_owned());

    let cur_task_res = LocalResource::new(move || async move { get_current_task().await.unwrap() });

    let throttle_fn = use_throttle_fn_with_options(
        move || {
            spawn_local(async move {
                mark_current_task_complete().await.unwrap();
                cur_task_res.refetch();
            });
        },
        1000.0,
        ThrottleOptions::default().trailing(false),
    );

    Effect::new(move || {
        let r = cur_task_res.get();
        if let Some(r) = r {
            cur_task.set(r.take())
        }
    });

    let cur_task_fn = move || {
        // stuff
        LayoutEntry {
            key: cur_task.get(),
            view_fn: Box::new(move || {
                view! {
                    <div>{cur_task}</div>
                }
                .into_any()
            }),
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
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server]
pub async fn get_current_task() -> Result<String, ServerFnError> {
    use crate::task_format::TaskList;

    let file_path = std::env::var("TASKS_FILE").unwrap();

    let task = TaskList::from(std::fs::read_to_string(&file_path).unwrap())
        .active_task()
        .unwrap_or_else(|| "-".to_owned());

    Ok(task)
}

#[server]
pub async fn mark_current_task_complete() -> Result<(), ServerFnError> {
    use crate::task_format::TaskList;

    let file_path = std::env::var("TASKS_FILE").unwrap();

    let mut list = TaskList::from(std::fs::read_to_string(&file_path).unwrap());
    list.advance();
    std::fs::write(&file_path, list.to_string()).unwrap();

    Ok(())
}
