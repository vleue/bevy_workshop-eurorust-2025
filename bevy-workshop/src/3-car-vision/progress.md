# Progress Report

## What You've learned

- Searching for third party plugins
  - [On the website](https://bevy.org/assets/)
  - [On Discord](https://discord.com/channels/691052431525675048/918591326096850974)
- Handling large assets
  - With [`RenderAssetUsages`](https://docs.rs/bevy/0.17.2/bevy/asset/struct.RenderAssetUsages.html) to control where the asset is stored
  - The importance of controlling the lifecycle of assets through their [`Handle`](https://docs.rs/bevy/0.17.2/bevy/asset/enum.Handle.html)
  - Loading a folder with [`AssetServer::load_folder`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.AssetServer.html#method.load_folder)
  - Checking an asset status with [`AssetServer::get_load_state`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.AssetServer.html#method.get_load_state)
- Displaying multiple cameras
  - Setting their viewport
  - Setting their order
  - Using [`RenderLayers` component](https://docs.rs/bevy/0.17.2/bevy/camera/visibility/struct.RenderLayers.html)
