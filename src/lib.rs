use yew::{html, function_component, Html, Children, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct NotificationsProviderProps {
    pub children: Children,
}

#[function_component(NotificationsProvider)]
pub fn notifications_provider(props: &NotificationsProviderProps) -> Html {
    let NotificationsProviderProps { children } = props.clone();

    html! {
        <div>
            {children}
            <div>
                // notifications will be here
            </div>
        </div>
    }
}
