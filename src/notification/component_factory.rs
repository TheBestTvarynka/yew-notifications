use yew::{html, Callback, Html, MouseEvent};

use crate::{NotifiableComponentFactory, Notification, NotificationComponent};

#[derive(Clone)]
pub struct NotificationFactory;

impl NotifiableComponentFactory<Notification> for NotificationFactory {
    fn component(
        &self,
        notification: Notification,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <NotificationComponent {notification} {onclick} {onenter} {onleave} />
        }
    }
}
