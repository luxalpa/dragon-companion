use crate::achievements::{Achievement, get_all_tasks};
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{StaticSegment, WildcardSegment};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/companion.css"/>

        // sets the document title
        <Title text="Dragon Companion"/>

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

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="main-content">
            <div class="header">
                <img src="/assets/Baryonyx.webp" alt="Baryonyx" class="monster-image card" />
            </div>
            <AchievementsListView />
        </div>
    }
}

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

#[component]
fn AchievementsListView() -> impl IntoView {
    let achievements = Resource::new(|| (), |_| async { get_all_tasks().await.unwrap() });

    let all_achievements = move || achievements.get().unwrap_or_default();
    fn key_fn(achievement: &Achievement) -> uuid::Uuid {
        achievement.id
    }

    fn children_fn(achievement: Achievement) -> impl IntoView {
        view! {
            <div class="achievement-card">
                {achievement.name}
            </div>
        }
    }

    view! {
        <Transition fallback=move || view! { <div>"Loading achievements..."</div> }>
            <div class="achievements-list">
                <For each=all_achievements key=key_fn children=children_fn />
            </div>
        </Transition>
    }
}
