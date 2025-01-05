struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
};

@group(2) @binding(0)
var shadow_map: texture_depth_2d;
@group(2) @binding(1)
var shadow_sampler: sampler_comparison;
@group(2) @binding(2)
var<uniform> shadow_uniforms: ShadowUniforms;

struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
};

struct LightUniform {
    position: vec4<f32>,
    color: vec4<f32>,
    direction: vec4<f32>,
    ambient: vec4<f32>,
    attenuation: vec4<f32>,
    params: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> light: LightUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) normal: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.world_position = model.position;
    out.normal = model.normal;
    out.color = model.color;
    return out;
}

fn gamma_correction(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0/2.2));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_space_position = shadow_uniforms.light_view_proj * vec4<f32>(in.world_position, 1.0);
    let shadow_coords = light_space_position.xyz / light_space_position.w;
    let shadow_xy = shadow_coords.xy * 0.5 + 0.5;
    
    let shadow_comparison = shadow_coords.z - 0.005;
    let shadow = textureSampleCompare(
        shadow_map,
        shadow_sampler,
        shadow_xy,
        shadow_comparison
    );

    let N = normalize(in.normal);
    let L = normalize(-light.direction.xyz);
    let V = normalize(camera.camera_pos.xyz - in.world_position);
    let H = normalize(L + V);
    
    let ambient = light.ambient.xyz * in.color;
    
    let wrap = 0.5;
    let NdotL = dot(N, L);
    let diffuse_strength = clamp((NdotL + wrap) / (1.0 + wrap), 0.0, 1.0);
    
    let diffuse = light.color.xyz * diffuse_strength * in.color;
    
    let NdotH = max(dot(N, H), 0.0);
    let specular_strength = pow(NdotH, light.params.y);
    let specular = light.color.xyz * specular_strength * 0.3;
    
    let distance = length(light.position.xyz - in.world_position);
    let attenuation = 1.0 / (light.attenuation.x + light.attenuation.y * distance + light.attenuation.z * distance * distance);
    
    var result = ambient;
    let lit = diffuse + specular;
    result += lit * shadow;
    
    result = gamma_correction(result);
    return vec4<f32>(result, 1.0);
}
