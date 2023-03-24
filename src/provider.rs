use yew::{function_component, html, use_reducer_eq, Callback, Children, ContextProvider, Html, Properties};

use crate::manager::{Action, NotificationsList};
use crate::NotificationsManager;

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationsProviderProps {
    pub children: Children,
}

#[function_component(NotificationsProvider)]
pub fn notifications_provider(props: &NotificationsProviderProps) -> Html {
    let notifications = use_reducer_eq(NotificationsList::default);

    let manager = NotificationsManager {
        sender: Some(notifications.dispatcher()),
    };

    let ns = notifications.notifications.clone();
    let children = props.children.clone();
    let dispatcher = notifications.dispatcher();
    html! {
        <ContextProvider<NotificationsManager> context={manager}>
            {children}
            {for ns.iter().map(|n| {
                let dispatcher = dispatcher.clone();
                let notification = n.clone();

                let onclick = Callback::from(move |_| {
                    dispatcher.dispatch(Action::Close(notification.clone()));
                });

                html!{ <span {onclick}>{n}</span> }
            })}
        </ContextProvider<NotificationsManager>>
    }
}
