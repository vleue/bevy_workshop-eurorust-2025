# Progress Report

## What You've learned

- Displaying and placing images in 2D
  - With the [`Sprite` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Sprite.html)
  - And the [`Transform` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Transform.html)
- Loading assets from files
  - With the [`AssetServer` resource](https://docs.rs/bevy/0.17.2/bevy/asset/struct.AssetServer.html)
- Creating your own components and resources with the derive
  - The [`Component` trait](https://docs.rs/bevy/0.17.2/bevy/ecs/component/trait.Component.html)
  - The [`Resource` trait](https://docs.rs/bevy/0.17.2/bevy/ecs/resource/trait.Resource.html)
- Reacting to user keyboard input
  - With the [`ButtonInput<KeyCode>` resource](https://docs.rs/bevy/0.17.2/bevy/input/struct.ButtonInput.html)
  - [`KeyCode`](https://docs.rs/bevy/0.17.2/bevy/input/keyboard/enum.KeyCode.html) for location-insensitive keyboard events
- Creating events, triggering them and observing them
  - Deriving the [`Event` trait](https://docs.rs/bevy/0.17.2/bevy/ecs/event/trait.Event.html)
  - Triggering them with [`Commands::trigger`](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Commands.html#method.trigger)
  - Reacting to them with an observer and the [`On` system parameter](https://docs.rs/bevy/0.17.2/bevy/ecs/observer/struct.On.html)
- Displaying text in 2D
  - With the [`Text2D` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Text2d.html)
- Writing a simple shader
  - Creating a material with the [`Material2d` trait](https://docs.rs/bevy/0.17.2/bevy/sprite_render/trait.Material2d.html)
  - Sending data to the shader with the [`AsBindGroup`](https://docs.rs/bevy/0.17.2/bevy/render/render_resource/trait.AsBindGroup.html) and [`ShaderType`](https://docs.rs/bevy/0.17.2/bevy/render/render_resource/trait.ShaderType.html) traits
  - Enabling it in Bevy with the [`Material2dPlugin` plugin](https://docs.rs/bevy/0.17.2/bevy/sprite_render/struct.Material2dPlugin.html)
