use yew::{function_component, html, Callback, Html};
use yew_notifications::{use_toaster, Notification, NotificationType, NotificationsProvider};

#[function_component(Inner)]
fn inner() -> Html {
    let notifications_manager = use_toaster();
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
    html! {
        <div>
            <NotificationsProvider>
                <span>{"example"}</span>
                <Inner />
            </NotificationsProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
