# Light History

Solutions for those exercises are behind the `light_history` feature.

## History Graph

We want to draw a graph of the light status over time. The remote server exposes the `get_history` function to get the history of a device. Bevy doesn't have (yet) a good way to draw graphs, but it's possible to work around that by using gizmos.

Tips:

- Add events to request the history, and get the response from the server
- Add an observer system that opens a channel, and starts a task to get the history
- Add a system that will poll that channel until it receives the history, then trigger an event with it
- Draw the history with gizmos [lines](https://docs.rs/bevy/0.17.2/bevy/gizmos/gizmos/struct.GizmoBuffer.html#method.line)
