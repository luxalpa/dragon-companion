use crate::app::{AppState, AppStateStoreFields};
use leptos::prelude::*;
use reactive_stores::Store;

pub fn open_unclaimed_achievements_dialog() {
    let state = expect_context::<Store<AppState>>();
    state.show_unclaimed_achievements_dialog().set(true);
}

#[component]
pub fn UnclaimedAchievementsDialog() -> impl IntoView {
    let state = expect_context::<Store<AppState>>();

    let close = move |_| {
        state.show_unclaimed_achievements_dialog().set(false);
    };

    view! {
        <div class="unclaimed-achievements-modal">
            <div class="modal-header">
                <h2>"Unclaimed Achievements"</h2>
                <button class="close-button" on:click=close>"×"</button>
            </div>
            <div class="modal-content">
                // Content for unclaimed achievements goes here
            </div>
        </div>
    }
}
