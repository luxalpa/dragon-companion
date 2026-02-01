use crate::achievements::Achievement;
use crate::requests::{DataRequest, RequestRunner, use_request};
use crate::update_achievement::{
    UpdateAchievementDialog, UpdateAchievementState, open_update_achievement_dialog,
};
use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_request_batcher::RequestBatcher;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::{StaticSegment, WildcardSegment};
use reactive_stores::{AtKeyed, Store};
use uuid::Uuid;

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

        <UpdateAchievementDialog />
    }
}

#[derive(Default, Store)]
pub struct AppState {
    pub show_unclaimed_achievements_dialog: bool,
    pub cur_update_achievement: Option<UpdateAchievementState>,

    #[store(key: uuid::Uuid = |a| a.id)]
    pub achievements: Vec<Achievement>,
}

#[component]
fn HomePage() -> impl IntoView {
    let runner = RequestRunner {
        store: expect_context::<Store<AppState>>(),
    };

    view! {
        <RequestBatcher runner>
            <div class="main-content">
                <div class="header">
                    <img src="/assets/smaug-mizzet-2_upscayl_2x_high-fidelity-4x.png" alt="Dragon" class="monster-image card" />
                </div>
                <AchievementsListView />
            </div>
        </RequestBatcher>
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

#[derive(Clone)]
enum AchievementListEntry {
    Achievement(AtKeyed<Store<AppState>, AppState, Uuid, Vec<Achievement>>),
    CreateNew,
}

impl AchievementListEntry {
    fn key(&self) -> AchievementListKey {
        match self {
            AchievementListEntry::Achievement(a) => AchievementListKey::Achievement(a.read().id),
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
    let state = expect_context::<Store<AppState>>();

    use_request(move || {
        state
            .achievements()
            .read_untracked()
            .is_empty()
            .then(|| DataRequest::GetAchievements)
    });

    let all_achievements = move || {
        state
            .achievements()
            .into_iter()
            .map(|a| AchievementListEntry::Achievement(a))
            .chain(std::iter::once(AchievementListEntry::CreateNew))
            .collect::<Vec<_>>()
    };
    fn key_fn(achievement: &AchievementListEntry) -> AchievementListKey {
        achievement.key()
    }

    fn children_fn(achievement: AchievementListEntry) -> impl IntoView {
        match achievement {
            AchievementListEntry::CreateNew => {
                let on_click = move |_| {
                    open_update_achievement_dialog(Achievement {
                        id: uuid::Uuid::new_v4(),
                        name: "".to_string(),
                        state: crate::achievements::TaskState::Completed(
                            chrono::Utc::now().naive_utc(),
                        ),
                    });
                };

                view! {
                    <div class="new-achievement-card" on:click=on_click>
                        "+"
                    </div>
                }
            }
            AchievementListEntry::Achievement(a) => {
                let name = move || a.read().name.clone();

                let on_click = {
                    let a = a.clone();
                    move |_| {
                        open_update_achievement_dialog(a.get());
                    }
                };

                view! {
                    <div class="achievement-card" on:click=on_click>
                        {name}
                    </div>
                }
            }
        }
    }

    view! {
        <div class="achievements-list">
            <For each=all_achievements key=key_fn children=children_fn />
        </div>
    }
}
