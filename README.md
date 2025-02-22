[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua/)

# yew-notifications

Table of content:

* [Motivation](#motivation)
* [How to use it](#how-to-use-it)
* [Moving forward](#moving-forward)
* [Meta](#meta)
* [Contributing](#contributing)

Notifications components library for [Yew](https://yew.rs/). It's like [react-toastify](https://www.npmjs.com/package/react-toastify) but for [yew](https://yew.rs/) and more simpler (so far :smirk:).

![deploy](https://github.com/TheBestTvarynka/yew-notifications/actions/workflows/github-actions.yml/badge.svg)

Documentation: https://docs.rs/yew-notifications/

![](/static/screenshots/basic_example.gif)

<sub>[:arrow_up: demo site](https://yn-docs.qkation.com/examples/basic/index.html)</sub>

![](/static/screenshots/terminal_example.gif)

<sub>[:arrow_up: demo site](https://yn-docs.qkation.com/examples/terminal/index.html)</sub>

Where it already used?

* [crypto-helper](https://crypto.qkation.com/)

### Motivation

I was writing my personal project [crypto-helper](https://github.com/TheBestTvarynka/crypto-helper/) some time ago. I was forced to write [awful code](https://github.com/TheBestTvarynka/crypto-helper/blob/8ad5ca3180925120a6f7ceb39253000f7ce3f447/src/notification.rs) to add some notifications functionality to my web app. So, I decided to write this library that allows the easy showing of notifications to users.

Inspired by `yew-toastrack`: https://github.com/kinnison/linkdoku/tree/main/yew-toastrack .

### How to use it

1. Decide which notification components to use. `yew-notifications` implements standard (default) notifications but you can write your own. See [`basic`](https://github.com/TheBestTvarynka/yew-notifications/tree/main/examples/basic) and [`custom`](https://github.com/TheBestTvarynka/yew-notifications/tree/main/examples/custom) examples for more information.

```toml
# Cargo.toml

# if you want to use standard notification components
yew-notifications = { version = "0.2", features = ["standard-notification"] }

# if you decide to write and use custom notification components
yew-notifications = "0.2"
```

2. Wrap needed components into `NotificationProvider`:

```rust
// Notification, NotificationFactory  - your notification types.
// They can be imported from this library or written by yourself (see `custom` example).
// component_creator is an instance of the NotificationFactory
<NotificationsProvider<Notification, NotificationFactory> {component_creator}>
    // some inner components
</NotificationsProvider<Notification, NotificationFactory>>
```

3. Spawn notifications:

```rust
use yew_notifications::use_notification;

// Notification - your notification type.
// It can be imported from this library or written by yourself (see `custom` example).
let notifications_manager = use_notification::<Notification>();
notifications_manager.spawn(Notification::new(...));
```

See the [`examples`](/examples/) directory for the code examples.

### Moving forward

At this point, this library has minimal functionality implemented. I plan to improve it continuously according to my needs. If you have any feature requests, then create an issue with the description. It'll be a priority for me.

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
