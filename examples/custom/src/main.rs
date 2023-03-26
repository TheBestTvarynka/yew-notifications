mod notification;

use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, TargetCast};
use yew_notifications::{use_notification, NotificationsProvider};

use crate::notification::factory::CustomNotificationFactory;
use crate::notification::CustomNotification;

#[function_component(Inner)]
fn inner() -> Html {
    let text = use_state(String::new);
    let text_setter = text.setter();
    let on_text_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        text_setter.set(value);
    });

    let notifications_manager = use_notification::<CustomNotification>();
    let text_value = (*text).clone();
    let onclick = Callback::from(move |_| {
        notifications_manager.spawn(CustomNotification::new(text_value.clone()));
    });

    html! {
        <div style="display:flex;flex-direction:column;gap:1em;width:30em">
            <input placeholder={"text"} oninput={on_text_input} />
            <button {onclick}>{"spawn"}</button>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = CustomNotificationFactory::default();

    html! {
        <div>
            <NotificationsProvider<CustomNotification, CustomNotificationFactory> {component_creator}>
                <Inner />
            </NotificationsProvider<CustomNotification, CustomNotificationFactory>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
