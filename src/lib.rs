mod hook;
mod manager;
mod notification;
mod provider;
mod utils;

use std::any::Any;
use std::fmt::Debug;

pub use hook::use_toaster;
pub use manager::NotificationsManager;
pub use notification::{Notification, NotificationComponent, NotificationComponentProps, NotificationType};
pub use provider::{NotificationsProvider, NotificationsProviderProps};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

pub trait Notifiable: Debug + Any {
    fn id(&self) -> Uuid;
    fn title(&self) -> Option<String>;
    fn text(&self) -> String;
    fn n_type(&self) -> &NotificationType;
    fn creation_time(&self) -> &OffsetDateTime;

    fn apply_tick(&mut self, time: Duration);
    fn is_alive(&self) -> bool;

    fn is_paused(&self) -> bool;
    fn mouse_in(&mut self);
    fn mouse_out(&mut self);

    fn boxed_clone(&self) -> Box<dyn Notifiable>;
    fn compare(&self, other: &Box<dyn Any>) -> bool;
}
