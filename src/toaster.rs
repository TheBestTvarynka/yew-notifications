use std::rc::Rc;

use yew::{Reducible, UseReducerDispatcher};

#[derive(Default, Clone, PartialEq)]
pub struct NotificationsManager {
    pub(crate) sender: Option<UseReducerDispatcher<NotificationsList>>,
}

impl NotificationsManager {
    pub fn spawn(&self, n: impl Into<String>) {
        log::debug!("spawn in NotificationManager");
        if let Some(sender) = &self.sender {
            log::debug!("sender is some");
            sender.dispatch(Action::NewNotification(n.into()));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    NewNotification(String),
    Close(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationsList {
    pub notifications: Vec<String>,
}

impl Reducible for NotificationsList {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        log::debug!("in reduce: {:?}", action);
        match action {
            Action::NewNotification(notification) => {
                log::debug!("new notification");

                let mut notifications = self.notifications.clone();
                notifications.push(notification);

                Rc::new(Self { notifications })
            }
            Action::Close(notification) => {
                log::info!("close {}", notification);
                self
            }
        }
    }
}
