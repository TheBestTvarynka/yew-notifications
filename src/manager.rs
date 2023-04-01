use std::fmt::Debug;
use std::rc::Rc;

use time::Duration;
use uuid::Uuid;
use yew::{Reducible, UseReducerDispatcher};

use crate::Notifiable;

/// Returned object from the `use_notification` hook. Can spawn new notifications.
#[derive(Clone, PartialEq)]
pub struct NotificationsManager<N: Notifiable + PartialEq + Clone> {
    pub(crate) sender: Option<UseReducerDispatcher<NotificationsList<N>>>,
}

impl<N: Notifiable + PartialEq + Clone> NotificationsManager<N> {
    /// Spawns new notification of the type N.
    pub fn spawn(&self, notification: N) {
        if let Some(sender) = &self.sender {
            sender.dispatch(Action::New(notification));
        }
    }
}

impl<N: Notifiable + PartialEq + Clone> Default for NotificationsManager<N> {
    fn default() -> Self {
        Self {
            sender: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Action<N: Notifiable + PartialEq + Clone> {
    New(N),
    Close(Uuid),
    Tick,
    Pause(Uuid),
    Continue(Uuid),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotificationsList<N> {
    pub notifications: Vec<N>,
}

impl<N> Default for NotificationsList<N> {
    fn default() -> Self {
        Self {
            notifications: Default::default(),
        }
    }
}

impl<N> NotificationsList<N> {
    pub const TIME_TICK_MILLIS: usize = 1000; // every second
    pub const TIME_TICK_DURATION: Duration = Duration::seconds(1);

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }
}

impl<N: Notifiable + PartialEq + Clone> Reducible for NotificationsList<N> {
    type Action = Action<N>;

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
