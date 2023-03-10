# Download hangs forever after network switch

Step by step:

1. `cargo run` - after compiling this will start a large download.
2. You will see progress in the terminal.
3. Now switch networks somehow - from WiFi to wired, or WiFi to another WiFi.
4. Progress will drop to 0 and keep going forever.

