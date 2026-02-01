use crate::achievements::{Achievement, get_all_tasks};
use crate::app::{AppState, AppStateStoreFields};
use leptos::prelude::*;
use leptos::server::codee::string::JsonSerdeCodec;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct RequestRunner {
    pub store: Store<AppState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataRequest {
    GetAchievements,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataResponse {
    Achievements(Vec<Achievement>),
}

impl leptos_request_batcher::RequestRunner for RequestRunner {
    type RequestEnum = DataRequest;
    type ResponseEnum = DataResponse;

    type Ser = JsonSerdeCodec;

    fn resolve_requests(&self, data: Vec<(Self::RequestEnum, Self::ResponseEnum)>) {
        for (request, response) in data {
            match (request, response) {
                (DataRequest::GetAchievements, DataResponse::Achievements(achievements)) => {
                    self.store.achievements().set(achievements);
                }
            }
        }
    }

    async fn exec_requests(&self, _requests: &[Self::RequestEnum]) -> Vec<Self::ResponseEnum> {
        vec![DataResponse::Achievements(get_all_tasks().await.unwrap())]
    }
}

pub fn use_request<I>(build_req: impl FnMut() -> I + Send + Sync + 'static)
where
    I: IntoIterator<Item: Into<DataRequest>> + Clone + Send + Sync + 'static,
{
    leptos_request_batcher::use_request::<RequestRunner, I>(build_req)
}
