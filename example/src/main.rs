use yew::{function_component, html, Html};
use yew_notifications::NotificationsProvider;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <NotificationsProvider>
                <span>{"example"}</span>
            </NotificationsProvider>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    yew::Renderer::<App>::new().render();
}
