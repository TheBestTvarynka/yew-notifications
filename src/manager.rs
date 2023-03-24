use std::rc::Rc;

use yew::{Reducible, UseReducerDispatcher};

#[derive(Default, Clone, PartialEq)]
pub struct NotificationsManager {
    pub(crate) sender: Option<UseReducerDispatcher<NotificationsList>>,
}

impl NotificationsManager {
    pub fn spawn(&self, n: impl Into<String>) {
        if let Some(sender) = &self.sender {
            sender.dispatch(Action::New(n.into()));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    New(String),
    Close(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationsList {
    pub notifications: Vec<String>,
}

impl Reducible for NotificationsList {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::New(notification) => {
                let mut notifications = self.notifications.clone();
                notifications.push(notification);

                Rc::new(Self { notifications })
            }
            Action::Close(notification) => {
                let mut notifications = self.notifications.clone();
                if let Some(index) = notification.find(&notification) {
                    notifications.remove(index);
                } else {
                    log::warn!("Got notification that doesn't exist: {:?}", notification);
                }

                Rc::new(Self { notifications })
            }
        }
    }
}
