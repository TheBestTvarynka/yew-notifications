mod component;
pub mod component_factory;

pub use component::{NotificationComponent, NotificationComponentProps};
pub use component_factory::NotificationFactory;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use yew::{classes, Classes};

use crate::Notifiable;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Warn,
    Error,
}

impl TryFrom<&str> for NotificationType {
    type Error = String;

    fn try_from(data: &str) -> Result<Self, <NotificationType as TryFrom<&str>>::Error> {
        match data {
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            invalid_type => Err(invalid_type.to_owned()),
        }
    }
}

impl From<&NotificationType> for Classes {
    fn from(notification_type: &NotificationType) -> Self {
        match notification_type {
            NotificationType::Info => classes!("info"),
            NotificationType::Warn => classes!("warn"),
            NotificationType::Error => classes!("error"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub(crate) id: Uuid,
    pub(crate) notification_type: NotificationType,
    pub(crate) title: Option<String>,
    pub(crate) text: String,

    pub(crate) spawn_time: OffsetDateTime,
    pub(crate) lifetime: Duration,
    pub(crate) paused: bool,
}

impl Notification {
    pub const NOTIFICATION_LIFETIME: Duration = Duration::seconds(3);

    pub fn new(notification_type: NotificationType, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: Some(title.into()),
            text: description.into(),

            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
            lifetime: Self::NOTIFICATION_LIFETIME,
            paused: false,
        }
    }

    pub fn from_description_and_type(notification_type: NotificationType, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: None,
            text: description.into(),

            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
            lifetime: Self::NOTIFICATION_LIFETIME,
            paused: false,
        }
    }

    pub fn with_title(self, new_title: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title: _,
            text: description,

            spawn_time,
            lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title: Some(new_title.into()),
            text: description,

            spawn_time,
            lifetime,
            paused,
        }
    }

    pub fn with_type(self, new_notification_type: NotificationType) -> Self {
        let Notification {
            id,
            notification_type: _,
            title,
            text: description,

            spawn_time,
            lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type: new_notification_type,
            title,
            text: description,

            spawn_time,
            lifetime,
            paused,
        }
    }

    pub fn with_description(self, new_description: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title,
            text: _,

            spawn_time,
            lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title,
            text: new_description.into(),

            spawn_time,
            lifetime,
            paused,
        }
    }
}

impl Notifiable for Notification {
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
        self.paused = true;
    }

    fn mouse_out(&mut self) {
        self.paused = false;
        self.lifetime = Self::NOTIFICATION_LIFETIME;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}
