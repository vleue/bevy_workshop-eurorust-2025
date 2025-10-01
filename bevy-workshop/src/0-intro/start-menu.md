# Start Menu

## Adding a Start Menu

We'll add a new plugin to handle the start menu. It will be very similar to the splash screen plugin, with different text and with a different condition to change state.

Tips:

- Create a new file for the new plugin, you can copy `splash.rs` as a starting point
- Change the state conditions and state scopes to `ApplicationState::Menu`
- Modify the text to display a start menu instead of a splash screen
- Add buttons with the [`Button` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Button.html), and make them interactive
  - See [the button Bevy example](https://bevy.org/examples/ui-user-interface/button/)

- Add the new plugin to the application
