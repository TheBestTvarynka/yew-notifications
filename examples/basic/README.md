
# Basic notifications

## What it is

This crate shows how to use the `yew-notifications` library and built-in notifications.

## Behaviour

Standard notifications have the following behaviour:

* disappeared after 4 seconds;
* can be disappeared by mouse click;
* can be of three types: `info`, `warn`, `error`;
* have title, text, and spawn (creation) time;
* if you hover over the notification it'll never disappear. When the mouse leaves the notification, the lifetime will be restarted (4 seconds);
