pub mod component;
pub mod factory;

use time::Duration;
use uuid::Uuid;
use yew_notifications::Notifiable;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomNotification {
    pub id: Uuid,
    pub text: String,

    is_paused: bool,
    is_alive: bool,
}

impl CustomNotification {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            text: description.into(),

            is_paused: false,
            is_alive: true,
        }
    }
}

impl Notifiable for CustomNotification {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply_tick(&mut self, _: Duration) {}

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn mouse_in(&mut self) {
        self.is_paused = true;
    }

    fn mouse_out(&mut self) {
        self.is_alive = false;
        self.is_paused = true;
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }
}
