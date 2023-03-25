use time::OffsetDateTime;
use uuid::Uuid;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

use crate::utils::format_date_time;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub(crate) id: Uuid,
    pub(crate) notification_type: NotificationType,
    pub(crate) title: Option<String>,
    pub(crate) description: String,
    pub(crate) spawn_time: OffsetDateTime,
}

impl Notification {
    pub fn new(notification_type: NotificationType, title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: Some(title.into()),
            description: description.into(),
            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
        }
    }

    pub fn from_description_and_type(notification_type: NotificationType, description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            notification_type,
            title: None,
            description: description.into(),
            spawn_time: OffsetDateTime::now_local().expect("Can not acquire local current time"),
        }
    }

    pub fn with_title(self, new_title: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title: _,
            description,
            spawn_time,
        } = self;

        Self {
            id,
            notification_type,
            title: Some(new_title.into()),
            description,
            spawn_time,
        }
    }

    pub fn with_type(self, new_notification_type: NotificationType) -> Self {
        let Notification {
            id,
            notification_type: _,
            title,
            description,
            spawn_time,
        } = self;

        Self {
            id,
            notification_type: new_notification_type,
            title,
            description,
            spawn_time,
        }
    }

    pub fn with_description(self, new_description: impl Into<String>) -> Self {
        let Notification {
            id,
            notification_type,
            title,
            description: _,
            spawn_time,
        } = self;

        Self {
            id,
            notification_type,
            title,
            description: new_description.into(),
            spawn_time,
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationComponentProps {
    pub notification: Notification,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(NotificationComponent)]
pub fn notification_component(props: &NotificationComponentProps) -> Html {
    let Notification {
        id: _,
        notification_type: _,
        title,
        description,
        spawn_time,
    } = &props.notification;

    let onclick = props.onclick.clone();

    html! {
        <div {onclick}>
            <span>{title.as_ref().map(|s| s.as_str()).unwrap_or_default()}</span>
            <span>{description}</span>
            <span>{format_date_time(&spawn_time)}</span>
        </div>
    }
}
