use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use super::CustomNotification;

#[derive(Properties, Clone, PartialEq)]
pub struct CustomNotificationComponentProps {
    pub notification: CustomNotification,
    pub onclick: Callback<MouseEvent>,
    pub onenter: Callback<MouseEvent>,
    pub onleave: Callback<MouseEvent>,
}

#[function_component(CustomNotificationComponent)]
pub fn custom_notification_component(props: &CustomNotificationComponentProps) -> Html {
    let text = &props.notification.text;

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes!("custom-notification")}>
            <span>{text}</span>
        </div>
    }
}
