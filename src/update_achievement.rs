use crate::achievements::{Achievement, delete_achievement, update_achievement};
use crate::app::{AppState, AppStateStoreFields};
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::on_click_outside;
use reactive_stores::{Store, StoreFieldIterator};

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

    let is_edit = Memo::new(move |_| {
        app_state
            .achievements()
            .read()
            .iter()
            .any(|a| a.id == state.achievement.id)
    });

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        app_state.cur_update_achievement().set(None);

        let achievement_name = value.get_untracked();
        let achievement = Achievement {
            id: state.achievement.id,
            name: achievement_name,
            state: state.achievement.state.clone(),
        };

        if is_edit.get_untracked() {
            app_state
                .achievements()
                .iter_unkeyed()
                .find(|a| a.read().id == achievement.id)
                .map(|a| {
                    a.set(achievement.clone());
                });
        } else {
            app_state.achievements().write().push(achievement.clone());
        }

        spawn_local(async move {
            update_achievement(achievement).await.unwrap();
        });
    };

    let on_input = move |event| {
        value.set(event_target_value(&event));
    };

    let title = move || {
        if is_edit.get() {
            "Update Achievement"
        } else {
            "Create Achievement"
        }
    };

    let delete_btn = move || {
        if is_edit.get() {
            let app_state = app_state.clone();
            let achievement_id = state.achievement.id;

            let on_click = move |_| {
                app_state
                    .achievements()
                    .write()
                    .retain(|a| a.id != achievement_id);
                app_state.cur_update_achievement().set(None);
                spawn_local(async move {
                    delete_achievement(achievement_id).await.unwrap();
                });
            };

            Some(view! {
                <button
                    class="delete-button"
                    on:click=on_click
                >
                    "Delete"
                </button>
            })
        } else {
            None
        }
    };

    view! {
        <div class="modal-wrapper">
            <div class="update-achievement-modal" node_ref=node>
                <div class="modal-header">
                    <h2>{title}</h2>
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
                        {delete_btn}
                        <button type="submit">"Save"</button>
                    </form>
                </div>
            </div>
        </div>
    }
}
