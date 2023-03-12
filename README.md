# Download hangs forever after network switch

Step by step:

1. `cargo run` - after compiling this will start a large download.
2. You will see progress in the terminal.
3. Now switch networks somehow - from WiFi to wired, or WiFi to another WiFi.
4. Progress will drop to 0 and keep going forever.

## What I've tried so far

### [Asked a question on URLO](https://users.rust-lang.org/t/help-tokio-copy-hangs-forever-if-network-is-switched-during-download/90678)

* Got the advice to set `tcp_keepalive` - didn't seem to help
* Got the advice to set `connect_timeout` - didn't seem to help
