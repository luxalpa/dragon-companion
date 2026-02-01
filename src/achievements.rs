use leptos::prelude::*;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AchievementList {
    achievements: Vec<Achievement>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Achievement {
    pub id: uuid::Uuid,
    pub name: String,
    pub state: TaskState,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub enum TaskState {
    #[default]
    Todo,
    Completed(chrono::NaiveDateTime),
}

impl Achievement {
    pub fn new(name: impl Into<String>) -> Self {
        Achievement {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            state: TaskState::Todo,
        }
    }
}

impl AchievementList {
    pub fn sample() -> Self {
        let list = [
            Achievement::new("Add Roma font"),
            Achievement::new("Add basic achievements sample"),
            Achievement::new("Implement CSS for achievements display"),
        ]
        .into_iter()
        .collect::<Vec<_>>();

        AchievementList { achievements: list }
    }
}

#[server]
pub async fn get_all_tasks() -> Result<Vec<Achievement>, ServerFnError> {
    Ok(AchievementList::sample().achievements)
}

#[server]
pub async fn update_achievement(achievement: Achievement) -> Result<(), ServerFnError> {
    // Here you would update the achievement in your data store.
    // For this example, we'll just log it.
    leptos::logging::log!("Updating achievement: {:?}", achievement);
    Ok(())
}
