use time::{Duration, OffsetDateTime};
use uuid::Uuid;
use yew::{classes, function_component, html, Callback, Classes, Html, MouseEvent, Properties};

use crate::utils::format_date_time;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Warn,
    Error,
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
    pub(crate) description: String,

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
            description: description.into(),

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
            description: description.into(),

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
            description,

            spawn_time,
            lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title: Some(new_title.into()),
            description,

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
            description,

            spawn_time,
            lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type: new_notification_type,
            title,
            description,

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
            description: _,

            spawn_time,
            lifetime,
            paused,
        } = self;

        Self {
            id,
            notification_type,
            title,
            description: new_description.into(),

            spawn_time,
            lifetime,
            paused,
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationComponentProps {
    pub notification: Notification,
    pub onclick: Callback<MouseEvent>,
    pub onenter: Callback<MouseEvent>,
    pub onleave: Callback<MouseEvent>,
}

#[function_component(NotificationComponent)]
pub fn notification_component(props: &NotificationComponentProps) -> Html {
    let Notification {
        id: _,
        notification_type,
        title,
        description,

        spawn_time,
        lifetime: _,
        paused,
    } = &props.notification;

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    let mut classes = vec![classes!("notification"), notification_type.into()];
    if *paused {
        classes.push(classes!("paused"));
    }

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes}>
            <span>{title.as_ref().map(|s| s.as_str()).unwrap_or_default()}</span>
            <span>{description}</span>
            <span class={classes!("time")}>{format_date_time(&spawn_time)}</span>
        </div>
    }
}
