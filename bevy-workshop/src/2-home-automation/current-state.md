# Displaying the Current State

Solutions for those exercises are behind the `current_state` feature.

## Make the lights reflects their current state in the scene

Lights should reflect their current state from the server point of view, in case the house inhabitants turn them on or off _without using our application_.

Add a system that will poll the `get_current_state` function from the fake remote server, and send events to the lights to update their state.

Tips:

- Add a system in the `Update` schedule to poll
- Use a [crossbeam channel](https://docs.rs/crossbeam/0.8.4/crossbeam/channel/index.html) to retrieve data out of the async task. Other synchronization primitives can be used if you prefer
  - Open the channel, start the task and use the sender in the task
  - keep the receiver in the system in a [`Local` system parameter](https://docs.rs/bevy/0.17.2/bevy/ecs/prelude/struct.Local.html): `Local<Option<Receiver<HashMap<Device, bool>>>>`
- If the channel is empty, the task is still running, so we can wait for it to finish and then try again later. Just exit the system right ahead
- If there is a message in the channel, the task has finished we can update the light state
  - Compare with the last known state to avoid unnecessary updates
