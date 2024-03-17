use std::marker::PhantomData;

use yew::{
    classes, function_component, html, use_effect_with, use_reducer_eq, Callback, Children, Classes, ContextProvider,
    Html, Properties,
};

use crate::manager::{Action, NotificationsList};
use crate::{Notifiable, NotifiableComponentFactory, NotificationsManager};

/// Notifications position on the screen
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationsPosition {
    /// Spawned notifications will be places at the top left corner of the screen
    TopLeft,
    /// Spawned notifications will be places at the top right corner of the screen
    TopRight,
    /// Spawned notifications will be places at the bottom right corner of the screen
    BottomRight,
    /// Spawned notifications will be places at the bottom left corner of the screen
    BottomLeft,
    /// Can be used to specify custom css class for the notifications container
    ///
    /// # Note
    /// The Custom class will overwrite any default provider CSS including style, position, etc.
    Custom(Classes),
}

impl From<&NotificationsPosition> for Vec<Classes> {
    fn from(position: &NotificationsPosition) -> Self {
        let position = match position {
            NotificationsPosition::TopLeft => classes!("notifications-provider-top-left"),
            NotificationsPosition::TopRight => classes!("notifications-provider-top-right"),
            NotificationsPosition::BottomRight => classes!("notifications-provider-bottom-right"),
            NotificationsPosition::BottomLeft => classes!("notifications-provider-bottom-left"),
            NotificationsPosition::Custom(classes) => return vec![classes.clone()],
        };
        vec![classes!("notifications"), position]
    }
}

impl From<&str> for NotificationsPosition {
    fn from(position: &str) -> Self {
        match position {
            "top-left" => Self::TopLeft,
            "top-right" => Self::TopRight,
            "bottom-left" => Self::BottomLeft,
            "bottom-right" => Self::BottomRight,
            p => Self::Custom(classes!(p.to_owned())),
        }
    }
}

/// Props for [`NotificationsProvider`]
#[derive(Properties, PartialEq, Clone)]
pub struct NotificationsProviderProps<N: Notifiable + PartialEq, F: NotifiableComponentFactory<N> + PartialEq + Clone> {
    /// Inner provider components
    pub children: Children,
    /// Instance of the component factory
    pub component_creator: F,
    /// Notifications position on the screen
    ///
    /// Default position is bottom right.
    #[prop_or(NotificationsPosition::BottomRight)]
    pub position: NotificationsPosition,
    #[prop_or_default]
    pub _notification: PhantomData<N>,
}

/// The notification provider component.
///
/// Every child (direct or indirect) of this component can use `use_notification` hook to spawn new notifications.
/// `N` - type of the notification.
/// `F` - notification factory type.
///
/// # Example
///
/// ```
/// let component_creator = NotificationFactory::default();
///
/// html! {
///     <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
///         <MyComponent />
///     </NotificationsProvider<Notification, NotificationFactory>>
/// }
/// ```
#[function_component(NotificationsProvider)]
pub fn notifications_provider<
    N: Notifiable + PartialEq + Clone,
    F: NotifiableComponentFactory<N> + PartialEq + Clone,
>(
    props: &NotificationsProviderProps<N, F>,
) -> Html {
    let notifications = use_reducer_eq(NotificationsList::<N>::default);

    let manager = NotificationsManager {
        sender: Some(notifications.dispatcher()),
    };

    use_effect_with(
        (!notifications.is_empty(), notifications.dispatcher()),
        |(is_active, sender)| {
            use gloo::timers::callback::Interval;

            let sender = sender.clone();
            let is_active = *is_active;

            let interval = Interval::new(NotificationsList::<N>::TIME_TICK_MILLIS as u32, move || {
                if is_active {
                    sender.dispatch(Action::Tick);
                }
            });

            move || drop(interval)
        },
    );

    let ns = notifications.notifications.clone();
    let children = props.children.clone();
    let dispatcher = notifications.dispatcher();

    let notification_creator = &props.component_creator;

    let classes: Vec<Classes> = (&props.position).into();

    html! {
        <ContextProvider<NotificationsManager<N>> context={manager}>
            {children}
            <div class={classes}>
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
        </ContextProvider<NotificationsManager<N>>>
    }
}
