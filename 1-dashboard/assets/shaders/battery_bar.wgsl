#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
}

struct Material {
    level: f32,
}

@group(2) @binding(0)
var<uniform> material: Material;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if abs(in.uv.x - 0.5) < 0.005 {
        return vec4(0.0, 0.7, 0.8, 1.0);
    }
    if in.uv.x < material.level {
        return mix(
            vec4(0.9, 0.1, 0.1, 1.0),
            vec4(0.1, 0.4, 0.9, 1.0),
            smoothstep(0.1, 0.75, material.level)
        );
    } else {
        return vec4(0.0, 0.0, 0.0, 0.0);
    }
}
