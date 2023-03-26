use web_sys::HtmlInputElement;
use yew::html::onchange::Event;
use yew::{function_component, html, use_state, Callback, Html, TargetCast};
use yew_notifications::{use_notification, Notification, NotificationFactory, NotificationType, NotificationsProvider};

#[function_component(Inner)]
fn inner() -> Html {
    let notification_type = use_state(|| NotificationType::Info);
    let notification_type_setter = notification_type.setter();
    let onchange = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(algorithm) = input.value().as_str().try_into() {
            notification_type_setter.set(algorithm);
        }
    });

    let title = use_state(String::new);
    let title_setter = title.setter();
    let on_title_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        title_setter.set(value);
    });

    let text = use_state(String::new);
    let text_setter = text.setter();
    let on_text_input = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        let value = input.value();

        text_setter.set(value);
    });

    let notifications_manager = use_notification::<Notification>();
    let notification_type_value = (*notification_type).clone();
    let title_value = (*title).clone();
    let text_value = (*text).clone();
    let onclick = Callback::from(move |_| {
        notifications_manager.spawn(Notification::new(
            notification_type_value.clone(),
            title_value.clone(),
            text_value.clone(),
        ));
    });

    html! {
        <div style="display:flex;flex-direction:column;gap:1em;width:30em">
            <select {onchange}>
                <option selected={true} value={"info"}>{"info"}</option>
                <option value={"warn"}>{"warn"}</option>
                <option value={"error"}>{"error"}</option>
            </select>
            <input placeholder={"title"} oninput={on_title_input} />
            <input placeholder={"text"} oninput={on_text_input} />
            <button {onclick}>{"spawn"}</button>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = NotificationFactory::default();

    html! {
        <div>
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
                <Inner />
            </NotificationsProvider<Notification, NotificationFactory>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}