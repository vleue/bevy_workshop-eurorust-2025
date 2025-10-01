# Faking a Remote Server

In this project, we need to call a remote server to interact with the connected lights. To simulate the server, we will use a static list of changes to the lights that can only be accessed through async functions.

You can find the implementation in <https://github.com/vleue/bevy_workshop-eurorust-2025/blob/main/2-home-automation/src/remote_server/internal.rs>.

The function signatures are:

```rust,ignore
# trait RemoteServer {
async fn get_current_state(current_time: u32) -> HashMap<Device, u32>;
async fn change_state(_device: Device, new_state: u32);
async fn get_history(device: Device) -> Vec<(u32, u32)>;
# }
```
