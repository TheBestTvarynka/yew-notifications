use yew::{html, Callback, Html, MouseEvent};
use yew_notifications::NotifiableComponentFactory;

use super::TerminalNotification;
use crate::notification::component::TerminalNotificationComponent;

#[derive(Clone, PartialEq, Default)]
pub struct TerminalNotificationFactory;

impl NotifiableComponentFactory<TerminalNotification> for TerminalNotificationFactory {
    fn component(
        &self,
        notification: TerminalNotification,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html {
        html! {
            <TerminalNotificationComponent {notification} {onclick} {onenter} {onleave} />
        }
    }
}
