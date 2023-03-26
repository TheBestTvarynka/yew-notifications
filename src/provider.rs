use std::marker::PhantomData;

use yew::{
    classes, function_component, html, use_effect_with_deps, use_reducer_eq, Callback, Children, ContextProvider, Html,
    Properties,
};

use crate::manager::{Action, NotificationsList};
use crate::{Notifiable, NotifiableComponentFactory, NotificationsManager};

/// Props for [`NotificationsProvider`]
#[derive(Properties, PartialEq, Clone)]
pub struct NotificationsProviderProps<T: Notifiable + PartialEq, F: NotifiableComponentFactory<T> + PartialEq + Clone> {
    pub children: Children,
    pub component_creator: F,
    #[prop_or_default]
    pub _notification: PhantomData<T>,
}

/// The notification provider component.
/// 
/// Every child (direct or indirect) of this component can use `use_notification` hook to spawn new notifications.
/// `T` - type of the notification.
/// `F` - notification factory type.
#[function_component(NotificationsProvider)]
pub fn notifications_provider<
    T: Notifiable + PartialEq + Clone,
    F: NotifiableComponentFactory<T> + PartialEq + Clone,
>(
    props: &NotificationsProviderProps<T, F>,
) -> Html {
    let notifications = use_reducer_eq(NotificationsList::<T>::default);

    let manager = NotificationsManager {
        sender: Some(notifications.dispatcher()),
    };

    use_effect_with_deps(
        |(is_active, sender)| {
            use gloo::timers::callback::Interval;

            let sender = sender.clone();
            let is_active = *is_active;

            let interval = Interval::new(NotificationsList::<T>::TIME_TICK_MILLIS as u32, move || {
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

    let notification_creator = &props.component_creator;

    html! {
        <ContextProvider<NotificationsManager<T>> context={manager}>
            {children}
            <div class={classes!("notifications")}>
                {for ns.iter().map(|n| {
                    let notification = n.clone();
                    let id = notification.id();

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

                    notification_creator.component(notification, onclick, onenter, onleave)
                })}
            </div>
        </ContextProvider<NotificationsManager<T>>>
    }
}
