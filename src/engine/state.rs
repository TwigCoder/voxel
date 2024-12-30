use super::{
    camera::{Camera, CameraController},
    renderer::Renderer,
};
use crate::world::{
    block::BlockType,
    chunk::{Chunk, CHUNK_SIZE},
};
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

        let mut chunk = Chunk::new();
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                chunk.set_block(x, 0, z, BlockType::Grass);
            }
        }

        chunk.set_block(5, 1, 5, BlockType::Wood);
        chunk.set_block(5, 2, 5, BlockType::Wood);
        chunk.set_block(5, 3, 5, BlockType::Wood);

        chunk.set_block(4, 4, 4, BlockType::Leaves);
        chunk.set_block(4, 4, 5, BlockType::Leaves);
        chunk.set_block(4, 4, 6, BlockType::Leaves);
        chunk.set_block(5, 4, 4, BlockType::Leaves);
        chunk.set_block(5, 4, 5, BlockType::Leaves);
        chunk.set_block(5, 4, 6, BlockType::Leaves);
        chunk.set_block(6, 4, 4, BlockType::Leaves);
        chunk.set_block(6, 4, 5, BlockType::Leaves);
        chunk.set_block(6, 4, 6, BlockType::Leaves);

        chunk.set_block(3, 1, 3, BlockType::Leaves);
        chunk.set_block(3, 1, 4, BlockType::Leaves);
        chunk.set_block(4, 1, 3, BlockType::Leaves);
        chunk.set_block(6, 1, 6, BlockType::Leaves);
        chunk.set_block(6, 1, 7, BlockType::Leaves);
        chunk.set_block(7, 1, 6, BlockType::Leaves);

        chunk.set_block(5, 1, 4, BlockType::Dirt);
        chunk.set_block(5, 1, 3, BlockType::Dirt);
        chunk.set_block(5, 1, 2, BlockType::Dirt);

        chunk.set_block(4, 1, 2, BlockType::Stone);
        chunk.set_block(6, 1, 2, BlockType::Stone);
        chunk.set_block(5, 1, 1, BlockType::Stone);

        chunk.set_block(3, 1, 2, BlockType::Sand);
        chunk.set_block(7, 1, 3, BlockType::Sand);

        chunk.set_block(2, 1, 5, BlockType::Snow);
        chunk.set_block(8, 1, 5, BlockType::Snow);
        chunk.set_block(5, 1, 8, BlockType::Snow);
        chunk.set_block(5, 1, 7, BlockType::Snow);

        chunk.set_block(0, 1, 0, BlockType::Stone);
        chunk.set_block(0, 2, 0, BlockType::Stone);
        chunk.set_block(1, 1, 0, BlockType::Stone);
        chunk.set_block(1, 2, 0, BlockType::Stone);
        chunk.set_block(0, 1, 1, BlockType::Stone);
        chunk.set_block(0, 2, 1, BlockType::Stone);
        chunk.set_block(1, 3, 0, BlockType::Stone);

        chunk.set_block(12, 1, 12, BlockType::Glass);
        chunk.set_block(13, 1, 12, BlockType::Glass);
        chunk.set_block(14, 1, 12, BlockType::Glass);
        chunk.set_block(12, 1, 13, BlockType::Glass);
        chunk.set_block(12, 1, 14, BlockType::Glass);
        chunk.set_block(14, 1, 13, BlockType::Glass);
        chunk.set_block(14, 1, 14, BlockType::Glass);
        chunk.set_block(13, 1, 14, BlockType::Glass);

        chunk.set_block(13, 1, 13, BlockType::Water);
        chunk.set_block(13, 2, 13, BlockType::Water);
        chunk.set_block(12, 1, 15, BlockType::Ice);
        chunk.set_block(13, 1, 15, BlockType::Ice);
        chunk.set_block(14, 1, 15, BlockType::Ice);

        chunk.set_block(10, 1, 10, BlockType::Stone);
        chunk.set_block(10, 2, 10, BlockType::CoalOre);
        chunk.set_block(11, 1, 10, BlockType::IronOre);
        chunk.set_block(10, 1, 11, BlockType::GoldOre);
        chunk.set_block(11, 1, 11, BlockType::DiamondOre);

        chunk.set_block(8, 1, 12, BlockType::Obsidian);
        chunk.set_block(9, 1, 12, BlockType::Obsidian);
        chunk.set_block(8, 1, 13, BlockType::Obsidian);
        chunk.set_block(8, 2, 12, BlockType::Lava);
        chunk.set_block(8, 2, 13, BlockType::Lava);

        let vertices = chunk.generate_mesh();
        renderer.update_vertices(&device, &vertices);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            camera,
            camera_controller,
            renderer,
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

        self.renderer
            .render(&view, &self.device, &self.queue, &self.camera)?;
        output.present();

        Ok(())
    }

    pub fn update_vertices(&mut self, vertices: &[super::renderer::Vertex]) {
        self.renderer.update_vertices(&self.device, vertices);
    }
}
