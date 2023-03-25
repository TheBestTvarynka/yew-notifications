use std::rc::Rc;

use time::Duration;
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
    Tick,
    Pause(Uuid),
    Continue(Uuid),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NotificationsList {
    pub notifications: Vec<Notification>,
}

impl NotificationsList {
    pub const TIME_TICK_MILLIS: usize = 1000; // every second
    pub const TIME_TICK_DURATION: Duration = Duration::seconds(1);

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }
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
            Action::Tick => {
                let notifications = self
                    .notifications
                    .clone()
                    .into_iter()
                    .filter_map(|mut n| {
                        if n.paused {
                            Some(n)
                        } else if n.lifetime >= Self::TIME_TICK_DURATION {
                            n.lifetime = n
                                .lifetime
                                .checked_sub(Self::TIME_TICK_DURATION)
                                .expect("lifetime should be long enough");

                            Some(n)
                        } else {
                            None
                        }
                    })
                    .collect();

                Rc::new(Self { notifications })
            }
            Action::Pause(id) => {
                let notifications = self
                    .notifications
                    .clone()
                    .into_iter()
                    .map(|mut n| {
                        if n.id == id {
                            n.paused = true;
                        }
                        n
                    })
                    .collect();

                Rc::new(Self { notifications })
            }
            Action::Continue(id) => {
                let notifications = self
                    .notifications
                    .clone()
                    .into_iter()
                    .map(|mut n| {
                        if n.id == id {
                            n.paused = false;
                            n.lifetime = Notification::NOTIFICATION_LIFETIME;
                        }
                        n
                    })
                    .collect();

                Rc::new(Self { notifications })
            }
        }
    }
}
