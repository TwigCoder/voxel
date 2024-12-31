use super::{
    camera::{Camera, CameraController},
    renderer::Renderer,
};
use crate::world::{
    block::{BlockPos, BlockType},
    chunk::{Chunk, ChunkPos, CHUNK_SIZE},
};
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
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
            
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
            Vec3::new(8.0, 16.0, 8.0),
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
        
        let render_distance = 8;
        let chunks = HashMap::new();
        let chunk_load_queue = VecDeque::new();
        let chunks_per_frame = 2;

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
        self.update_chunks();
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
                println!("Rendering chunk at {:?}", pos);
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
        
        for y in -self.render_distance/2..=self.render_distance/2 {
            for x in -self.render_distance..=self.render_distance {
                for z in -self.render_distance..=self.render_distance {
                    let chunk_pos = ChunkPos::new(
                        camera_chunk_pos.x + x,
                        camera_chunk_pos.y + y,
                        camera_chunk_pos.z + z,
                    );
                    
                    let distance = ((x * x + y * y + z * z) as f32).sqrt();
                    
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
        
        self.chunk_load_queue.extend(new_load_requests);
        
        for _ in 0..self.chunks_per_frame {
            if let Some(request) = self.chunk_load_queue.pop_front() {
                if !self.chunks.contains_key(&request.pos) {
                    
                    let mut chunk = Chunk::new(request.pos.to_world_pos());
                    chunk.generate_terrain(request.pos.to_world_pos());
                    
                    if request.pos.y == -1 {
                        for x in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                chunk.set_block(x, CHUNK_SIZE-1, z, BlockType::Grass);
                                
                                if rand::random::<f32>() < 0.1 {
                                    chunk.set_block(x, CHUNK_SIZE, z, BlockType::Grass);
                                }
                            }
                        }
                    }
                    
                    self.chunks.insert(request.pos, chunk);
                }
            }
        }
        
        self.chunks.retain(|pos, _|chunks_to_keep.contains(pos)); 
    }   
}

