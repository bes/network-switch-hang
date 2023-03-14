# Download hangs forever after network switch

This repository started out as a question: Why does a `reqwest` download stall
forever when switching between WiFi / Cellular networks on macOS or iOS?

Which prompted [this thread on URLO](https://users.rust-lang.org/t/help-tokio-copy-hangs-forever-if-network-is-switched-during-download/90678)
and this [StackOverflow question](https://stackoverflow.com/questions/75711940/why-does-a-reqwest-response-hang-when-switching-wifi-networks-on-macos-ios),
and this discussion on the [reqwest GitHub Discussions](https://github.com/seanmonstar/reqwest/discussions/1776) forum.

The reason it stalls is that the underlying network stack simply really wants the
TCP connection to succeed, and doesn't throw any errors.

I have mitigated the problem in this repostory by using an AsyncRead wrapper that
[I found on StackOverflow](https://stackoverflow.com/questions/60621835/how-to-get-callback-update-when-using-tokioiocopy?rq=1)
and modified it to return a Polling error when a certain timeout has been reached.
