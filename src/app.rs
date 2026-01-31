use crate::achievements::{Achievement, get_all_tasks};
use crate::unclaimed_achievements::open_unclaimed_achievements_dialog;
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{StaticSegment, WildcardSegment};
use reactive_stores::Store;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(Store::new(AppState::default()));

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

#[derive(Clone, Default, Store)]
pub struct AppState {
    pub show_unclaimed_achievements_dialog: bool,
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="main-content">
            <div class="header">
                <img src="/assets/smaug-mizzet-2.png" alt="Dragon" class="monster-image card" />
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

#[derive(Clone, Debug)]
enum AchievementListEntry {
    Achievement(Achievement),
    CreateNew,
}

impl AchievementListEntry {
    fn key(&self) -> AchievementListKey {
        match self {
            AchievementListEntry::Achievement(a) => AchievementListKey::Achievement(a.id),
            AchievementListEntry::CreateNew => AchievementListKey::CreateNew,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum AchievementListKey {
    Achievement(uuid::Uuid),
    CreateNew,
}

#[component]
fn AchievementsListView() -> impl IntoView {
    let achievements = Resource::new(|| (), |_| async { get_all_tasks().await.unwrap() });

    let all_achievements = move || {
        achievements
            .get()
            .unwrap_or_default()
            .into_iter()
            .map(|a| AchievementListEntry::Achievement(a))
            .chain(std::iter::once(AchievementListEntry::CreateNew))
            // .rev()
            .collect::<Vec<_>>()
    };
    fn key_fn(achievement: &AchievementListEntry) -> AchievementListKey {
        achievement.key()
    }

    fn children_fn(achievement: AchievementListEntry) -> impl IntoView {
        match achievement {
            AchievementListEntry::CreateNew => view! {
                <div class="new-achievement-card" on:click=move |_| {
                    open_unclaimed_achievements_dialog();
                }>
                    "+"
                </div>
            },
            AchievementListEntry::Achievement(a) => view! {
                <div class="achievement-card">
                    {a.name}
                </div>
            },
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
