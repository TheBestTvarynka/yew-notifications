use yew::{hook, use_context};

use crate::manager::NotificationsManager;

#[hook]
pub fn use_toaster() -> NotificationsManager {
    use_context::<NotificationsManager>().unwrap_or_else(NotificationsManager::default)
}
