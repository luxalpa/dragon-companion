use crate::achievements::{Achievement, update_achievement};
use crate::app::{AppState, AppStateStoreFields};
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::on_click_outside;
use reactive_stores::Store;

#[derive(Clone)]
pub struct UpdateAchievementState {
    achievement: Achievement,
}

pub fn open_update_achievement_dialog(achievement: Achievement) {
    let state = expect_context::<Store<AppState>>();
    state
        .cur_update_achievement()
        .set(Some(UpdateAchievementState { achievement }));
}

#[component]
pub fn UpdateAchievementDialog() -> impl IntoView {
    let state = expect_context::<Store<AppState>>();

    move || {
        state.cur_update_achievement().get().map(|state| {
            view! {
                <UpdateAchievementDialogContents state />
            }
        })
    }
}

#[component]
fn UpdateAchievementDialogContents(state: UpdateAchievementState) -> impl IntoView {
    let node = NodeRef::<html::Div>::new();
    let value = RwSignal::new(state.achievement.name);

    let app_state = expect_context::<Store<AppState>>();

    _ = on_click_outside(node, move |_event| {
        app_state.cur_update_achievement().set(None);
    });

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        app_state.cur_update_achievement().set(None);

        let achievement_name = value.get();
        let achievement = Achievement {
            id: state.achievement.id,
            name: achievement_name,
            state: state.achievement.state.clone(),
        };

        app_state
            .achievements()
            .update(|v| v.push(achievement.clone()));

        spawn_local(async move {
            update_achievement(achievement).await.unwrap();
        });
    };

    let on_input = move |event| {
        value.set(event_target_value(&event));
    };

    view! {
        <div class="modal-wrapper">
            <div class="update-achievement-modal" node_ref=node>
                <div class="modal-header">
                    <h2>"Update Achievement"</h2>
                </div>
                <div class="modal-content">
                    <form on:submit=submit>
                        <label for="achievement-name">"Name:"</label>
                        <input
                            type="text"
                            id="achievement-name"
                            name="achievement-name"
                            prop:value=value.read_only()
                            on:input=on_input
                        />
                        <button type="submit">"Save"</button>
                    </form>
                </div>
            </div>
        </div>
    }
}
