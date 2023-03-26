
# Custom notifications

### What it is

`yew-notifications` has all the needed notification structure and a notification component. But sometimes we need altogether custom notifications, for example, with another style, behaviour, etc. This crate shows how to write custom notifications.

This custom notifications have:

* different notification style;
* notification will not be disappeared in time;
* notification will be removed only in two ways: on click or when the mouse leaves the notification area.

### How to write custom notifications

1. Create notification structure (it'll contain all needed notification data like title, text, etc, maybe logo) and implement the `Notifiable` trait. For example, `CustomNotification`.
2. Create a yew component that will represent one separate notification. For example, `CustomNotificationComponent`.
3. Create a notification factory structure (it'll generate yew component for every newly spawned notification) and implement the `NotifiableComponentFactory` trait. For example, `CustomNotificationFactory`.
4. Specify your new types in the `NotificationProvider` component:
```Rust
// component_creator is an instance of the CustomNotificationFactory
<NotificationsProvider<CustomNotification, CustomNotificationFactory> {component_creator}>
    // some inner components
</NotificationsProvider<CustomNotification, CustomNotificationFactory>>
```

### How to run this demo:

1. Install [`trunk`](https://github.com/thedodd/trunk). [Additional guide](https://yew.rs/docs/next/getting-started/introduction#install-trunk).
2. Run `trunk serve` in your terminal.
3. Go to http://127.0.0.1:8080 in your browser.
