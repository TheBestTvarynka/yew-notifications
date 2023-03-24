use time::OffsetDateTime;
use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

use crate::utils::format_date_time;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum NotificationType {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Notification {
    notification_type: NotificationType,
    title: String,
    description: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationComponentProps {
    notification: Notification,
    onclick: Callback<MouseEvent>,
}

#[function_component(NotificationComponent)]
pub fn notification_component(props: &NotificationComponentProps) -> Html {
    let Notification {
        notification_type,
        title,
        description,
    } = &props.notification;

    let onclick = props.onclick.clone();

    html! {
        <div {onclick}>
            <span>{title}</span>
            <span>{description}</span>
            <span>{OffsetDateTime::now_local().map(|now_time| format_date_time(&now_time)).unwrap_or_default()}</span>
        </div>
    }
}
