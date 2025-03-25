use std::sync::RwLock;

pub struct CustomData {
    task_id: RwLock<bool>,
}

impl CustomData {
    pub fn task_id(&self) -> bool {
        self.task_id.read().unwrap().clone()
    }

    pub fn advance(&self) {
        let prev = self.task_id.read().unwrap().clone();
        *self.task_id.write().unwrap() = !prev;
    }

    pub fn new() -> Self {
        Self {
            task_id: RwLock::new(false),
        }
    }
}
