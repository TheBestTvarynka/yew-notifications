mod notification;

pub use notification::{BaseNotification, Notification};

use yew::{function_component, hook, html, Children, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct NotificationsProviderProps {
    pub children: Children,
}

#[function_component(NotificationsProvider)]
pub fn notifications_provider(props: &NotificationsProviderProps) -> Html {
    let NotificationsProviderProps { children } = props.clone();

    html! {
        <div>
            {children}
            <div>
                // notifications will be here
            </div>
        </div>
    }
}

#[hook]
pub fn use_notification(notifications: Vec<Box<dyn Notification>>) -> UseNotificationHandle {
    UseNotificationHandle { notifications }
}

pub struct UseNotificationHandle {
    notifications: Vec<Box<dyn Notification>>,
}
