use yew::{
    classes, function_component, html, use_effect_with_deps, use_reducer_eq, Callback, Children, ContextProvider, Html,
    Properties,
};

use crate::manager::{Action, NotificationsList};
use crate::{NotificationComponent, NotificationsManager};

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

    use_effect_with_deps(
        |(is_active, sender)| {
            use gloo::timers::callback::Interval;

            let sender = sender.clone();
            let is_active = *is_active;

            let interval = Interval::new(NotificationsList::TIME_TICK_MILLIS as u32, move || {
                if is_active {
                    sender.dispatch(Action::Tick);
                }
            });

            move || drop(interval)
        },
        (!notifications.is_empty(), notifications.dispatcher()),
    );

    let ns = notifications.notifications.clone();
    let children = props.children.clone();
    let dispatcher = notifications.dispatcher();

    html! {
        <ContextProvider<NotificationsManager> context={manager}>
            {children}
            <div class={classes!("notifications")}>
                {for ns.iter().map(|n| {
                    let notification = n.clone();
                    let id = notification.id;

                    let onclick = {
                        let dispatcher = dispatcher.clone();
                        Callback::from(move |_| {
                            dispatcher.dispatch(Action::Close(id));
                        })
                    };

                    let onenter = {
                        let dispatcher = dispatcher.clone();
                        Callback::from(move |_| {
                            dispatcher.dispatch(Action::Pause(id));
                        })
                    };

                    let onleave = {
                        let dispatcher = dispatcher.clone();
                        Callback::from(move |_| {
                            dispatcher.dispatch(Action::Continue(id));
                        })
                    };

                    html!{
                        <NotificationComponent {notification} {onclick} {onenter} {onleave} />
                    }
                })}
            </div>
        </ContextProvider<NotificationsManager>>
    }
}
