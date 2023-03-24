use yew::{function_component, html, use_reducer_eq, Children, ContextProvider, Html, Properties};

use crate::{toaster::NotificationsList, NotificationsManager};

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationsProviderProps {
    pub children: Children,
}

#[function_component(NotificationsProvider)]
pub fn notifications_provider(props: &NotificationsProviderProps) -> Html {
    log::debug!("render provider");
    let notifications = use_reducer_eq(NotificationsList::default);

    let manager = NotificationsManager {
        sender: Some(notifications.dispatcher()),
    };

    let ns = notifications.notifications.clone();
    log::debug!("ns: {:?}", ns);
    let c = props.children.clone();
    html! {
        <ContextProvider<NotificationsManager> context={manager}>
            {for ns.iter().map(|n| html!{ <span>{n}</span> }) }
            {c}
        </ContextProvider<NotificationsManager>>
    }
}
