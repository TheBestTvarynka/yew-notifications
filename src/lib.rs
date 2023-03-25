mod hook;
mod manager;
mod notification;
mod provider;
mod utils;

use std::any::Any;
use std::fmt::Debug;

pub use hook::use_notification;
pub use manager::NotificationsManager;
pub use notification::{Notification, NotificationComponent, NotificationComponentProps, NotificationType};
pub use provider::{NotificationsProvider, NotificationsProviderProps};
use time::Duration;
use uuid::Uuid;
use yew::{Callback, Html, MouseEvent};

pub trait Notifiable: Debug + Any {
    fn id(&self) -> Uuid;

    fn apply_tick(&mut self, time: Duration);
    fn is_alive(&self) -> bool;

    fn is_paused(&self) -> bool;
    fn mouse_in(&mut self);
    fn mouse_out(&mut self);
}

pub trait NotifiableComponentFactory<T: Notifiable> {
    fn component(
        &self,
        notification: T,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html;
}
