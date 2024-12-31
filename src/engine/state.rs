use super::{
    camera::{Camera, CameraController},
    renderer::Renderer,
};
use crate::world::{
    block::BlockType,
    chunk::{Chunk, CHUNK_SIZE},
};
use crate::utils::frustum::Frustum;
use glam::Vec3;
use winit::{event::WindowEvent, window::Window};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub camera: Camera,
    camera_controller: CameraController,
    renderer: Renderer,
    chunks: Vec<Chunk>
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

        Self {
            surface,
            device,
            queue,
            config,
            size,
            camera,
            camera_controller,
            renderer,
            chunks,
        }
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
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let frustum = Frustum::from_matrix(self.camera.build_view_projection_matrix());
        
        let mut all_vertices = Vec::new();
        for chunk in &self.chunks {
            let (min, max) = chunk.get_bounds();
            if frustum.is_box_visible(min, max) {
                all_vertices.extend(chunk.generate_mesh());
            }
        }
        
        self.renderer
            .render(&view, &self.device, &self.queue, &self.camera)?;
        output.present();

        Ok(())
    }

    pub fn update_vertices(&mut self, vertices: &[super::renderer::Vertex]) {
        self.renderer.update_vertices(&self.device, vertices);
    }
}
