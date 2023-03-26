use std::fmt::Debug;
use std::rc::Rc;

use time::Duration;
use uuid::Uuid;
use yew::{Reducible, UseReducerDispatcher};

use crate::Notifiable;

#[derive(Clone, PartialEq)]
pub struct NotificationsManager<T: Notifiable + PartialEq + Clone> {
    pub(crate) sender: Option<UseReducerDispatcher<NotificationsList<T>>>,
}

impl<T: Notifiable + PartialEq + Clone> NotificationsManager<T> {
    pub fn spawn(&self, notification: T) {
        if let Some(sender) = &self.sender {
            sender.dispatch(Action::New(notification));
        }
    }
}

impl<T: Notifiable + PartialEq + Clone> Default for NotificationsManager<T> {
    fn default() -> Self {
        Self {
            sender: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Action<T: Notifiable + PartialEq + Clone> {
    New(T),
    Close(Uuid),
    Tick,
    Pause(Uuid),
    Continue(Uuid),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationsList<T: Notifiable + PartialEq + Clone> {
    pub notifications: Vec<T>,
}

impl<T: Notifiable + PartialEq + Clone> Default for NotificationsList<T> {
    fn default() -> Self {
        Self {
            notifications: Default::default(),
        }
    }
}

impl<T: Notifiable + PartialEq + Clone> NotificationsList<T> {
    pub const TIME_TICK_MILLIS: usize = 1000; // every second
    pub const TIME_TICK_DURATION: Duration = Duration::seconds(1);

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }
}

impl<T: Notifiable + PartialEq + Clone> Reducible for NotificationsList<T> {
    type Action = Action<T>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::New(notification) => {
                let mut notifications = self.notifications.clone();
                notifications.push(notification);

                Rc::new(Self { notifications })
            }
            Action::Close(id) => {
                let notifications = self
                    .notifications
                    .clone()
                    .into_iter()
                    .filter(|n| n.id() != id)
                    .collect();

                Rc::new(Self { notifications })
            }
            Action::Tick => {
                let notifications = self
                    .notifications
                    .clone()
                    .into_iter()
                    .filter_map(|mut n| {
                        if n.is_paused() {
                            Some(n)
                        } else if n.is_alive() {
                            n.apply_tick(Self::TIME_TICK_DURATION);

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
                        if n.id() == id {
                            n.mouse_in();
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
                        if n.id() == id {
                            n.mouse_out();
                        }
                        n
                    })
                    .collect();

                Rc::new(Self { notifications })
            }
        }
    }
}
