@group(0) @binding(0)
var t_translucent_color: texture_2d<f32>;

@group(0) @binding(1)
var t_translucent_depth: texture_depth_2d;

@group(0) @binding(2)
var t_scene_depth: texture_depth_2d;

const TRANSLUCENT_PLAYER_ALPHA: f32 = 0.5;

struct FullscreenVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> FullscreenVertexOutput {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(3.0, 1.0),
    );

    var out: FullscreenVertexOutput;
    out.clip_position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let coord = vec2<i32>(position.xy);
    let color = textureLoad(t_translucent_color, coord, 0);

    if color.a <= 0.0 {
        discard;
    }

    let translucent_depth = textureLoad(t_translucent_depth, coord, 0);
    if translucent_depth <= 0.0 {
        discard;
    }

    let scene_depth = textureLoad(t_scene_depth, coord, 0);
    if scene_depth > translucent_depth {
        discard;
    }

    return vec4<f32>(color.rgb, color.a * TRANSLUCENT_PLAYER_ALPHA);
}
