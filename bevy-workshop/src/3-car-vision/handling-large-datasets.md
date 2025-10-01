# Handling Large Datasets

## Large Assets in Bevy

Bevy has some built-in capabilities to handle large assets, but is still limited

- Assets can be preprocessed and stored in a more efficient format
- Assets can be loaded in chunks to reduce memory usage
- Some assets can be marked as CPUonly or GPU-only with [`RenderAssetUsages`](https://docs.rs/bevy/0.17.2/bevy/asset/struct.RenderAssetUsages.html)
- Already loaded assets will be reused
- Dropping all handles to an asset will remove it from memory

With some limitations:

- Asset streaming is not yet possible
- Asset errors are hard to handle as they happen asynchronously
- Loading a file that is already loading won't reuse it

## Playing an Entire KITTI Recording

Let's play an entire KITTI recording! They are made of individual point cloud files for each frame.

The sample dataset available is large enough that loading it takes some time and consumes memory, but small enough that it is possible to do and keep everything in memory.

You are free to decide of your implementation strategy:

- Load everything first, then play
- Load each frame as needed, then play, and keep it for replay
- Load each frame as needed, then play, then unload

Tips:

- You can use [`AssetServer::load_folder`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.AssetServer.html#method.load_folder) to load every file in a folder. See [this example](https://github.com/bevyengine/bevy/blob/release-0.17.2/examples/2d/texture_atlas.rs) on how to use it.
- You can keep the asset `Handle`s already loaded in a resource
- You should check that a pointcloud is finished loading before displaying it to avoid blinking with [`AssetServer::get_load_state`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.AssetServer.html#method.get_load_state)
- The files are available at `format!("kitti-2011_09_26_drive_0005_sync/velodyne/{:0>10}.laz", current_frame))`
