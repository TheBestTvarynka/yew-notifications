pub mod component;
pub mod factory;
mod utils;

use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use yew_notifications::Notifiable;

#[derive(Debug, Clone, PartialEq)]
pub struct TerminalNotification {
    pub id: Uuid,
    pub title: String,
    pub text: String,

    spawn_time: OffsetDateTime,
    lifetime: Duration,
    full_lifetime: Duration,

    is_paused: bool,
    is_alive: bool,
}

impl TerminalNotification {
    const LIFETIME: Duration = Duration::seconds(6);

    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            text: description.into(),

            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
            lifetime: Self::LIFETIME,
            full_lifetime: Self::LIFETIME,

            is_paused: false,
            is_alive: true,
        }
    }
}

impl Notifiable for TerminalNotification {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply_tick(&mut self, time: Duration) {
        self.lifetime = self.lifetime.checked_sub(time).unwrap_or(Duration::default());
    }

    fn is_alive(&self) -> bool {
        self.lifetime != Duration::default()
    }

    fn mouse_in(&mut self) {
        self.is_paused = true;
    }

    fn mouse_out(&mut self) {
        self.is_paused = false;
        self.lifetime = self.full_lifetime;
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }
}
