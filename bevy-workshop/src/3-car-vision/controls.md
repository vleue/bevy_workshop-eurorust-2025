# Controls

## Setting Up egui

We want to be able to control the playback of the point cloud data. By pausing or by manually selecting a frame to display.

We will use [egui](https://www.egui.rs) for this application as it has a rich widget library and is easy to use.

It has a plugin for Bevy, [`bevy_egui`](https://vladbat00.github.io/bevy_egui/ui/). We can add the [`EguiPlugin`](https://docs.rs/bevy_egui/0.37.0/bevy_egui/struct.EguiPlugin.html) to our app.

As we have multiple cameras, we need to help egui know to which camera it should render to. We will need to set the `auto_create_primary_context` field to `false` on the [`EguiGlobalSettings` resource](https://docs.rs/bevy_egui/0.37.0/bevy_egui/struct.EguiGlobalSettings.html), and add the [`PrimaryEguiContext` component](https://docs.rs/bevy_egui/0.37.0/bevy_egui/struct.PrimaryEguiContext.html) to the camera we choose.

## Rendering the UI

bevy_egui is rendering in immediate mode. We will need a system in the [`EguiPrimaryContextPass` schedule](https://docs.rs/bevy_egui/0.37.0/bevy_egui/struct.EguiPrimaryContextPass.html) to draw the UI.

```rust
# extern crate bevy;
# extern crate bevy_ecs;
# extern crate bevy_egui;
# use bevy::{
#     camera::{Viewport, visibility::RenderLayers},
#     prelude::*,
#     render::view::NoIndirectDrawing,
#     window::WindowResized,
# };
# use bevy_egui::{
#     EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui,
# };
#[derive(Resource)]
struct Play {
    current_frame: u32,
    frame_count: u32,
    playing: bool,
}

fn controls(
    mut contexts: EguiContexts,
    mut play: ResMut<Play>,
) -> Result {
    egui::TopBottomPanel::top("Controls").show(contexts.ctx_mut()?, |ui| {
        ui.horizontal(|ui| {
            let frame_count = play.frame_count;
            ui.add(egui::Slider::new(&mut play.current_frame, 0..=frame_count).text("Frame"));
            if ui
                .button(if play.playing { "Pause" } else { "Play" })
                .clicked()
            {
                play.playing = !play.playing;
            }
        });
    });
    Ok(())
}
```
