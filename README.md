# yew-notifications

Notifications components library for [Yew](https://yew.rs/). It's like [react-toastify](https://www.npmjs.com/package/react-toastify) but for [yew](https://yew.rs/) and more simpler (so far :smirk:).

### Motivation

I was writing my personal project [crypto-helper](https://github.com/TheBestTvarynka/crypto-helper/) some time ago. I was forced to write [awful code](https://github.com/TheBestTvarynka/crypto-helper/blob/8ad5ca3180925120a6f7ceb39253000f7ce3f447/src/notification.rs) to add some notifications functionality to my web app. So, I decided to write this library that allows the easy showing of notifications to users.

Inspired by `yew-toastr`: https://github.com/kinnison/linkdoku/tree/main/yew-toastrack .

### How to use it

1. Decide which notification components to use. `yew-notifications` already has implemented standard notifications but you can write your own. See [`basic`](https://github.com/TheBestTvarynka/yew-notifications/tree/main/examples/basic) and [`custom`](https://github.com/TheBestTvarynka/yew-notifications/tree/main/examples/custom) examples for more information.
2. Include `yew-notification` styles into your project (only if you use notification components from `yew-notifications`):
```HTML
<link data-trunk rel="sass" href="https://raw.githubusercontent.com/TheBestTvarynka/yew-notifications/main/static/notification.scss" />
```
Or you can copy this file into your project and specify the path to it instead of the link. At this point, the *scss* file is one possible way to import styles.

3. Wrap needed components into `NotificationProvider`:
```Rust
// Notification, NotificationFactory  - your notification types.
// They can be imported from this library or written by yourself (see `custom` example).
// component_creator is an instance of the NotificationFactory
<NotificationsProvider<Notification, NotificationFactory> {component_creator}>
    // some inner components
</NotificationsProvider<Notification, NotificationFactory>>
```
4. Spawn notifications:
```Rust
use yew_notifications::use_notification;

// Notification - your notification type.
// It can be imported from this library or written by yourself (see `custom` example).
let notifications_manager = use_notification::<Notification>();
notifications_manager.spawn(Notification::new(...));
```

### Meta

[Pavlo Myroniuk](https://github.com/TheBestTvarynka) - [the.best.tvarynka@gmail.com](mailto:the.best.tvarynka@gmail.com).

Distributed under the [MIT](https://github.com/TheBestTvarynka/yew-notifications/blob/main/LICENSE) license.

### Contributing

Feel free to contribute.

1. Fork it (<https://github.com/TheBestTvarynka/yew-notifications/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request
