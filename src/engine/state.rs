use super::{
    camera::{Camera, CameraController},
    renderer::Renderer,
};
use crate::world::{
    block::{BlockPos, BlockType},
    chunk::{Chunk, ChunkPos, CHUNK_SIZE},
};
use crate::engine::light::Light;
use crate::utils::frustum::Frustum;
use glam::Vec3;
use winit::{event::WindowEvent, window::Window};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct ChunkLoadRequest {
    pos: ChunkPos,
    priority: f32,
}

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub camera: Camera,
    camera_controller: CameraController,
    renderer: Renderer,
    chunks: HashMap<ChunkPos, Chunk>,
    chunk_load_queue: VecDeque<ChunkLoadRequest>,
    render_distance: i32,
    chunks_per_frame: usize,
    last_chunk_pos: Option<ChunkPos>,
    time: f32,
    light: Light,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let time = 0.0;
            
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let camera = Camera::new(
            Vec3::new(8.0, 20.0, 8.0),
            size.width as f32 / size.height as f32,
        );
        let camera_controller = CameraController::new(0.5);
        let mut renderer = Renderer::new(&device, &config, &camera);

        let mut chunks = Vec::new();
        
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let position = Vec3::new(
                        x as f32 * CHUNK_SIZE as f32,
                        y as f32 * CHUNK_SIZE as f32,
                        z as f32 * CHUNK_SIZE as f32
                    );
                    let mut chunk = Chunk::new(position);
                
                if y == -1 {
                    for cx in 0..CHUNK_SIZE {
                        for cz in 0..CHUNK_SIZE {
                            chunk.set_block(cx, CHUNK_SIZE-1, cz, BlockType::Grass);
                        }
                    }
                }
                chunks.push(chunk);
            }
            }
        }
        
        let mut renderer = Renderer::new(&device, &config, &camera);
        
        let mut initial_vertices = Vec::new();
        for chunk in &chunks {
            initial_vertices.extend(chunk.generate_mesh());
        }
        renderer.update_vertices(&device, &initial_vertices);
        
        let render_distance = 16;
        let chunks = HashMap::new();
        let chunk_load_queue = VecDeque::new();
        let chunks_per_frame = 256;
        let last_chunk_pos = None;
        
        let light = Light::new(
            Vec3::new(0.0, 100.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-0.5, -1.0, -0.3),
        );

        let mut state = Self {
            surface,
            device,
            queue,
            config,
            size,
            camera,
            camera_controller,
            renderer,
            chunks,
            chunk_load_queue,
            render_distance,
            chunks_per_frame,
            last_chunk_pos,
            time,
            light,
        };
        
        state.update_chunks();
        state
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller
            .process_events(event, &mut self.camera)
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        
        let current_chunk_pos = ChunkPos::from_world_pos(self.camera.position);
        if self.last_chunk_pos.map_or(true, |pos| pos != current_chunk_pos) {
            self.last_chunk_pos = Some(current_chunk_pos);
            self.update_chunks();
        }
        
        self.time += 0.05; // TODO: MAKE SLOWER AFTER TESTING
        let sun_angle = self.time % (2.0 * std::f32::consts::PI);
        let sun_height = sun_angle.sin();
        let sun_distance = sun_angle.cos();
        
        self.light.direction = Vec3::new(sun_distance, -sun_height, 0.0).normalize();
        let day_intensity = (sun_height + 1.0) * 0.5;
        self.light.update();
        self.renderer.update_light_buffer(&self.queue, &self.light.uniform);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let frustum = Frustum::from_matrix(self.camera.build_view_projection_matrix());
        
        let mut all_vertices = Vec::new();
        for (pos, chunk) in &self.chunks {
            let (min, max) = chunk.get_bounds();
            if frustum.is_box_visible(min, max) {
                all_vertices.extend(chunk.generate_mesh());
            }
        }
        
        self.renderer.update_vertices(&self.device, &all_vertices);
        self.renderer
            .render(&view, &self.device, &self.queue, &self.camera)?;
        output.present();

        Ok(())
    }

    pub fn update_vertices(&mut self, vertices: &[super::renderer::Vertex]) {
        self.renderer.update_vertices(&self.device, vertices);
    }
    
    pub fn update_chunks(&mut self) {
        let camera_chunk_pos = ChunkPos::from_world_pos(self.camera.position);
        let mut chunks_to_keep = HashSet::new();
        let mut new_load_requests: Vec<ChunkLoadRequest> = Vec::new();
        
        for y in -self.render_distance/4..=self.render_distance/4 {
            for x in -self.render_distance..=self.render_distance {
                for z in -self.render_distance..=self.render_distance {
                    let chunk_pos = ChunkPos::new(
                        camera_chunk_pos.x + x,
                        camera_chunk_pos.y + y,
                        camera_chunk_pos.z + z,
                    );
                    
                    let distance = ((x * x + y * y * 4 + z * z) as f32).sqrt();
                    
                    if distance <= self.render_distance as f32 {
                        chunks_to_keep.insert(chunk_pos);
                        
                        if !self.chunks.contains_key(&chunk_pos) {
                            new_load_requests.push(ChunkLoadRequest {
                                pos: chunk_pos,
                                priority: distance,
                            })
                        }
                    }
                }
            }
        }
        
        new_load_requests.sort_by(|a, b|
            a.priority.partial_cmp(&b.priority).unwrap()
        );
        new_load_requests.sort_by(|a, b| {
            let a_dist = (a.pos.x - camera_chunk_pos.x).pow(2) + (a.pos.z - camera_chunk_pos.z).pow(2);
            let b_dist = (b.pos.x - camera_chunk_pos.x).pow(2) + (b.pos.z - camera_chunk_pos.z).pow(2);
            a_dist.cmp(&b_dist)
        });
        
        self.chunk_load_queue.clear();
        self.chunk_load_queue.extend(new_load_requests);
        
        for _ in 0..self.chunks_per_frame {
            if let Some(request) = self.chunk_load_queue.pop_front() {
                if !self.chunks.contains_key(&request.pos) {
                    
                    let mut chunk = Chunk::new(request.pos.to_world_pos());
                    chunk.generate_terrain(request.pos.to_world_pos());
                    
                    self.chunks.insert(request.pos, chunk);
                }
            }
        }
        
        self.chunks.retain(|pos, _|chunks_to_keep.contains(pos)); 
    }   
}

