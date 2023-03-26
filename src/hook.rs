use yew::{hook, use_context};

use crate::manager::NotificationsManager;
use crate::Notifiable;

/// This hook is used to manage notifications.
/// Returned [`NotificationsManager`] can be used to spawn new notifications of the type `T`.
/// 
/// # Example
/// 
/// ```rust
/// use yew_notifications::{Notification, use_notification};
/// 
/// let notifications_manager = use_notification::<Notification>();
/// notifications_manager.spawn(Notification::new(/* */));
/// ```
#[hook]
pub fn use_notification<T: Notifiable + PartialEq + Clone>() -> NotificationsManager<T> {
    use_context::<NotificationsManager<T>>().unwrap_or_default()
}
