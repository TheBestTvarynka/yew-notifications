use yew::{hook, use_context};

use crate::manager::NotificationsManager;
use crate::Notifiable;

#[hook]
pub fn use_toaster<T: Notifiable + PartialEq + Clone + Default>() -> NotificationsManager<T> {
    use_context::<NotificationsManager<T>>().unwrap_or_else(NotificationsManager::default)
}
