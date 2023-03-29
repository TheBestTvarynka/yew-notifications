mod component;
pub mod component_factory;

pub use component::{NotificationComponent, NotificationComponentProps};
pub use component_factory::NotificationFactory;
use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use yew::{classes, Classes};

use crate::Notifiable;

/// Standard notification type.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationType {
    /// Represents some informative message for the user.
    #[default]
    Info,

    /// Represents some warning.
    Warn,

    /// Represents some error message.
    Error,

    /// Custom notification type.
    ///
    /// You can use this option when you want to set the custom style of your notification
    /// but don't want to write an entire custom component from scratch.
    Custom(Classes),
}

impl From<&str> for NotificationType {
    fn from(data: &str) -> Self {
        match data {
            "info" => Self::Info,
            "warn" => Self::Warn,
            "error" => Self::Error,
            data => Self::Custom(data.to_owned().into()),
        }
    }
}

impl From<&NotificationType> for Classes {
    fn from(notification_type: &NotificationType) -> Self {
        match notification_type {
            NotificationType::Info => classes!("info"),
            NotificationType::Warn => classes!("warn"),
            NotificationType::Error => classes!("error"),
            NotificationType::Custom(classes) => classes.clone(),
        }
    }
}

/// Standard notification.
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub(crate) id: Uuid,
    pub(crate) notification_type: NotificationType,
    pub(crate) title: Option<String>,
    pub(crate) text: String,

    pub(crate) spawn_time: OffsetDateTime,
    pub(crate) lifetime: Duration,
    pub(crate) full_lifetime: Duration,
    pub(crate) paused: bool,
}

impl Notification {
    pub const NOTIFICATION_LIFETIME: Duration = Duration::seconds(3);

    /// Creates a new standard notification from notification type, title, text, and lifetime duration.
    pub fn new(
        notification_type: NotificationType,
        title: impl Into<String>,
        text: impl Into<String>,
        lifetime: Duration,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: Some(title.into()),
            text: text.into(),

            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
            lifetime,
            full_lifetime: lifetime,
            paused: false,
        }
    }

    /// Creates a new standard notification from notification type and text.
    ///
    /// Title will be omitted. Notification lifetime is equal to the [`Self::NOTIFICATION_LIFETIME`].
    pub fn from_description_and_type(notification_type: NotificationType, text: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: None,
            text: text.into(),

            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
            lifetime: Self::NOTIFICATION_LIFETIME,
            full_lifetime: Self::NOTIFICATION_LIFETIME,
            paused: false,
        }
    }

    /// Set the title for the notification.
    pub fn with_title(self, new_title: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title: _,
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title: Some(new_title.into()),
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        }
    }

    /// Set the type for the notification.
    pub fn with_type(self, new_notification_type: NotificationType) -> Self {
        let Notification {
            id,
            notification_type: _,
            title,
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type: new_notification_type,
            title,
            text: description,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        }
    }

    /// Set the text for the notification.
    pub fn with_text(self, new_text: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title,
            text: _,

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title,
            text: new_text.into(),

            spawn_time,
            lifetime,
            full_lifetime,
            paused,
        }
    }

    /// Resets notification lifetime.
    ///
    /// It means that after this method invocation, the lifetime of the notification will be equal to the start value.
    pub fn reset_lifetime(self) -> Self {
        let Notification {
            id,
            notification_type,
            title,
            text,

            spawn_time,
            lifetime: _,
            full_lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title,
            text,

            spawn_time,
            lifetime: full_lifetime,
            full_lifetime,
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
        self.lifetime = self.full_lifetime;
    }

    fn is_paused(&self) -> bool {
        self.paused
    }
}
