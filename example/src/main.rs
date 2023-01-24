use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <span>{"example"}</span>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    yew::Renderer::<App>::new().render();
}
