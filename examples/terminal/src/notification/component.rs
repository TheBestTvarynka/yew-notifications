use yew::{classes, function_component, html, Callback, Html, MouseEvent, Properties};

use crate::notification::utils::format_time;

use super::TerminalNotification;

#[derive(Properties, Clone, PartialEq)]
pub struct TerminalNotificationComponentProps {
    pub notification: TerminalNotification,
    pub onclick: Callback<MouseEvent>,
    pub onenter: Callback<MouseEvent>,
    pub onleave: Callback<MouseEvent>,
}

#[function_component(TerminalNotificationComponent)]
pub fn terminal_notification_component(props: &TerminalNotificationComponentProps) -> Html {
    let spawn_time = &props.notification.spawn_time;
    let text = &props.notification.text;
    let title = &props.notification.title;
    let lifetime = &props.notification.lifetime;

    let onclick = props.onclick.clone();
    let onenter = props.onenter.clone();
    let onleave = props.onleave.clone();

    html! {
        <div {onclick} onmouseenter={onenter} onmouseleave={onleave} class={classes!("terminal-notification")}>
            <span class={classes!("time")}>{format_time(&spawn_time.time())}</span>
            <span class={classes!("title")}>{title}</span>
            <span class={classes!("text")}>{text}</span>
            <span class={classes!("dots")}>{String::from_utf8(vec![b'.'; lifetime.as_seconds_f32() as usize]).unwrap()}</span>
        </div>
    }
}
