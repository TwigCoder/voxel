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
    
    var result = ambient + (diffuse + specular) * attenuation * light.params.x;
    
    let height_ao = clamp(in.world_position.y / 32.0, 0.5, 1.0);
    let time_factor = max(dot(vec3<f32>(0.0, 1.0, 0.0), L), 0.0);
    let night_color = vec3<f32>(0.1, 0.1, 0.3);
    result = mix(night_color * ambient, result, time_factor);
    
    result = gamma_correction(result);
    return vec4<f32>(result, 1.0);
}
