use yew::{hook, use_context};

use crate::toaster::NotificationsManager;

#[hook]
pub fn use_toaster() -> NotificationsManager {
    log::debug!("use_toaster call");
    use_context::<NotificationsManager>().unwrap_or_else(NotificationsManager::default)
}
