#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
}

struct Material {
    progress: f32,
}

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.uv.x < material.progress {
        return vec4(0.2, 0.3, 0.8, 1.0);
    } else {
        return vec4(0.2, 0.2, 0.2, 1.0);
    }
}
