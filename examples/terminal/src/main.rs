mod notification;

use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, TargetCast};
use yew_notifications::{use_notification, NotificationsPosition, NotificationsProvider};

use crate::notification::factory::TerminalNotificationFactory;
use crate::notification::TerminalNotification;

#[function_component(Inner)]
fn inner() -> Html {
    let title = use_state(|| String::from("title"));
    let title_setter = title.setter();
    let on_title_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        title_setter.set(value);
    });

    let text = use_state(|| String::from("text"));
    let text_setter = text.setter();
    let on_text_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        text_setter.set(value);
    });

    let notifications_manager = use_notification::<TerminalNotification>();
    let title_value = (*title).clone();
    let text_value = (*text).clone();
    let onclick = Callback::from(move |_| {
        notifications_manager.spawn(TerminalNotification::new(title_value.clone(), text_value.clone()));
    });

    html! {
        <div style="display:flex;flex-direction:column;gap:1em;width:30em">
            <input placeholder={"title"} oninput={on_title_input} />
            <input placeholder={"text"} oninput={on_text_input} />
            <button {onclick}>{"spawn"}</button>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = TerminalNotificationFactory;
    let position = NotificationsPosition::Custom("terminal-notifications".into());

    html! {
        <div>
            <NotificationsProvider<TerminalNotification, TerminalNotificationFactory> {component_creator} {position}>
                <Inner />
                <div style="height: 200vh"></div>
            </NotificationsProvider<TerminalNotification, TerminalNotificationFactory>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
