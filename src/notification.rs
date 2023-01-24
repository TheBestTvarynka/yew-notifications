use yew::{html, Html};

pub trait Notification {
    fn component(&self) -> Html;
}

#[derive(Clone, Debug)]
pub enum NotificationType {
    Info,
    Warn,
    Error,
}

impl ToString for NotificationType {
    fn to_string(&self) -> String {
        match self {
            NotificationType::Info => "Into".into(),
            NotificationType::Warn => "Warn".into(),
            NotificationType::Error => "Error".into(),
        }
    }
}

pub struct BaseNotification {
    notification_type: NotificationType,
    title: Option<String>,
    text: String,
}

impl BaseNotification {
    pub fn new(notification_type: NotificationType, title: Option<String>, text: String) -> Self {
        Self {
            notification_type,
            title,
            text,
        }
    }
}

impl Notification for BaseNotification {
    fn component(&self) -> Html {
        html! {
            <div>
                <span>{self.notification_type.clone()}</span>
                {if let Some(title) = self.title.as_ref() {
                    html!{ <span>{title.clone()}</span> }
                } else {
                    html! {}
                }}
                <span>{self.text.clone()}</span>
            </div>
        }
    }
}
