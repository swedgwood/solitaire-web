# Solitaire

Solitaire, written in Rust using [Yew](https://yew.rs/).

I just copied the rules from the app I was using, which referred to it as "Klondike (draw 3)".

I haven't had much experience in the whole component way of doing frontend, neither much experience with WASM, neither solitaire. So this code probably isn't that good.

Yes, it was probably a better idea to use a canvas, and Yew was probably not even needed, but who cares, this was just for fun and learning.

## Possible improvements

- Might be able to clean up animations a bit more (although I think I've reached the limit of what is possible with the current architecture)
- Mobile controls *work* (tap card to pick up, tap card to place), but could probably be improved
- There is no coded win condition, and also no kind of victory message.
