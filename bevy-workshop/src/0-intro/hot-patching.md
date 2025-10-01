# Hot-Patching

Bevy has built-in support for hot-patching thanks to [Dioxus's subsecond](https://crates.io/crates/subsecond).

## Install the Dioxus CLI

```sh
cargo install dioxus-cli@0.7.0-rc.0
```

## Run your app through `dx`

```sh
dx serve --hot-patch --features "bevy/hotpatching"
```

## Modify your code

Change anything in a system that runs in the `Update` schedule.

## Current limitations and workarounds

- Only works for code in the binary crate being run.
- Not supported in Wasm.
- Can't change system parameters.
- Can't change schedules, or add / remove systems.
- Can fail for some configurations of toolchains/linkers.
- Systems in non-`Update` schedules are hot-patched, but you need to have a way to re-enter their schedule
  - Not possible for system in `Startup` schedule.
  - For systems in `OnEnter(State)` schedules, you can exit then re-enter the state.

Bevy sends an [`HotPatched`](https://docs.rs/bevy/0.17.2/bevy/ecs/struct.HotPatched.html) message when an hot-patch is applied. You can react to this event to trigger systems or state changes.

This feature is new in Bevy 0.17 and we hope to improve it in the future, to support more use cases.

In large projects, hot-patching can take longer, or not work because of the workspace being separated in multiple crates. Early adopters have found it helpful to have a small crate with the code they're currently changing, and once they're happy to port it back in their main workspace.

## And assets

There is also hot-reloading for assets, on file change, with the `file_watcher` feature.
