//! yew-notifications notifications components library for [Yew](https://yew.rs/).
//! It's like [react-toastify](https://www.npmjs.com/package/react-toastify) but for [yew](https://yew.rs/) and more simpler.
//! 
//! The purpose of this library is to provide the ability to easily add notifications to the yew web app.
//! 
//! # Examples
//! 
//! * [basic](https://github.com/TheBestTvarynka/yew-notifications/tree/main/examples/basic): shows how to use the `yew-notifications` library and built-in notifications.
//! * [custom](https://github.com/TheBestTvarynka/yew-notifications/tree/main/examples/custom): shows how to write custom notifications.

mod hook;
mod manager;
#[cfg(feature = "standard-notification")]
mod notification;
mod provider;
mod utils;

use std::any::Any;

pub use hook::use_notification;
pub use manager::NotificationsManager;

#[cfg(feature = "standard-notification")]
pub use notification::{
    Notification, NotificationComponent, NotificationComponentProps, NotificationFactory, NotificationType,
};

pub use provider::{NotificationsProvider, NotificationsProviderProps};
use time::Duration;
use uuid::Uuid;
use yew::{Callback, Html, MouseEvent};

/// This trait provides an interface for the notification. Everything,
/// that implements this trait, can be used in the [`NotificationsProvider`].
/// 
/// # Lifetime
/// 
/// Every notification has such thing as *lifetime*.
/// This is simply the amount of time that this notification is still "alive" (which means is present on the screen or is visible).
/// Methods like [`Notifiable::apply_tick`], [`Notifiable::is_alive`], etc, are used by library internal to control life of the notification.
pub trait Notifiable: Any {
    /// Returns the id of the notification. Every notification has the id of the Uuid type and it should be unique.
    fn id(&self) -> Uuid;


    /// Applies some amount of time to this notification.
    /// 
    /// # Arguments
    /// 
    /// * `time` - An amount of time that has been spent.
    fn apply_tick(&mut self, time: Duration);

    /// Check if the notification is still "alive".
    /// If it returns false, then this notification will be deleted (disappeared) on the next time tick.
    fn is_alive(&self) -> bool;

    /// Check if the notification is still "paused".
    /// It means, that when notification is paused (this function returns true)
    /// then time does not affect the notification lifetime.
    fn is_paused(&self) -> bool;

    /// This function calls when the mouse enters this notification.
    fn mouse_in(&mut self);

    /// This function calls when the mouse leaves this notification.
    fn mouse_out(&mut self);
}

/// This trait provides an interface for the notification component factory.
/// 
/// This trait implementors are used be `yew-notifications` for notification components rendering.
pub trait NotifiableComponentFactory<T: Notifiable> {
    /// Creates a new notification component that can be rendered in `yew`.
    fn component(
        &self,
        notification: T,
        onclick: Callback<MouseEvent>,
        onenter: Callback<MouseEvent>,
        onleave: Callback<MouseEvent>,
    ) -> Html;
}
