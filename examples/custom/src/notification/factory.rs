use yew::{html, Callback, Html, MouseEvent};
use yew_notifications::NotifiableComponentFactory;

use super::CustomNotification;
use crate::notification::component::CustomNotificationComponent;

#[derive(Clone, PartialEq, Default)]
pub struct CustomNotificationFactory;

impl NotifiableComponentFactory<CustomNotification> for CustomNotificationFactory {
    fn component(
        &self,
        notification: CustomNotification,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <CustomNotificationComponent {notification} {onclick} {onenter} {onleave} />
        }
    }
}
