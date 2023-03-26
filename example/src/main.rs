use yew::{function_component, html, Callback, Html};
use yew_notifications::{use_notification, Notification, NotificationFactory, NotificationType, NotificationsProvider};

#[function_component(Inner)]
fn inner() -> Html {
    let notifications_manager = use_notification::<Notification>();
    let onclick = Callback::from(move |_| {
        log::debug!("onclick: spawn");

        notifications_manager.spawn(Notification::new(
            NotificationType::Info,
            "some title",
            "some description",
        ));
    });

    html! {
        <button {onclick}>{"spawn"}</button>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let component_creator = NotificationFactory::default();

    html! {
        <div>
            <NotificationsProvider<Notification, NotificationFactory> {component_creator}>
                <span>{"example"}</span>
                <Inner />
            </NotificationsProvider<Notification, NotificationFactory>>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
