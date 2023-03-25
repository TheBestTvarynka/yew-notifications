use std::rc::Rc;

use uuid::Uuid;
use yew::{Reducible, UseReducerDispatcher};

use crate::Notification;

#[derive(Default, Clone, PartialEq)]
pub struct NotificationsManager {
    pub(crate) sender: Option<UseReducerDispatcher<NotificationsList>>,
}

impl NotificationsManager {
    pub fn spawn(&self, notification: Notification) {
        if let Some(sender) = &self.sender {
            sender.dispatch(Action::New(notification));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    New(Notification),
    Close(Uuid),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationsList {
    pub notifications: Vec<Notification>,
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
            Action::Close(id) => {
                let notifications = self.notifications.clone().into_iter().filter(|n| n.id != id).collect();

                Rc::new(Self { notifications })
            }
        }
    }
}
