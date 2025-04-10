use super::{
    camera::{Camera, CameraController},
    renderer::Renderer,
};
use crate::engine::light::Light;
use crate::utils::frustum::Frustum;
use crate::world::chunk::{Chunk, ChunkPos};
use crate::world::chunk_worker::ChunkWorkerPool;
use glam::Vec3;
use parking_lot::Mutex;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
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
    chunks: Arc<Mutex<HashMap<ChunkPos, Chunk>>>,
    chunk_worker: ChunkWorkerPool,
    render_distance: i32,
    last_chunk_pos: Option<ChunkPos>,
    time: f32,
    light: Light,
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
            Vec3::new(8.0, 80.0, 8.0),
            size.width as f32 / size.height as f32,
        );
        let camera_controller = CameraController::new(0.5);
        let renderer = Renderer::new(&device, &config, &camera);

        let chunks = Arc::new(Mutex::new(HashMap::new()));
        let chunk_worker = ChunkWorkerPool::new(Arc::clone(&chunks));

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
            chunk_worker,
            render_distance: 8,
            last_chunk_pos: None,
            time: 0.0,
            light: Light::new(
                Vec3::new(0.0, 100.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(-0.5, -1.0, -0.3),
            ),
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
        if self
            .last_chunk_pos
            .map_or(true, |pos| pos != current_chunk_pos)
        {
            self.last_chunk_pos = Some(current_chunk_pos);
            self.update_chunks();

            for _ in 0..4 {
                self.chunk_worker.process_tasks();
            }
        } else {
            self.chunk_worker.process_tasks();
        }

        self.time += 0.01;
        let sun_angle = self.time % (2.0 * std::f32::consts::PI);
        let sun_height = sun_angle.sin();
        let sun_distance = sun_angle.cos();

        self.light.direction = Vec3::new(sun_distance, -sun_height, 0.0).normalize();
        self.light.update();
        self.renderer
            .update_light_buffer(&self.queue, &self.light.uniform);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let frustum = Frustum::from_matrix(self.camera.build_view_projection_matrix());

        let mut vertices = Vec::new();
        let chunks_lock = self.chunks.lock();
        for chunk in chunks_lock.values() {
            let (min, max) = chunk.get_bounds();
            if frustum.is_box_visible(min, max) {
                vertices.extend(chunk.generate_mesh());
            }
        }
        drop(chunks_lock);

        self.renderer.update_vertices(&self.device, &vertices);
        self.renderer
            .render(&view, &self.device, &self.queue, &self.camera)?;
        output.present();

        Ok(())
    }

    pub fn update_chunks(&mut self) {
        let camera_chunk_pos = ChunkPos::from_world_pos(self.camera.position);
        let mut chunks_to_keep = HashSet::new();

        let horizontal_distance = 6;
        let vertical_distance = 2;

        for y in -vertical_distance..=vertical_distance {
            for x in -horizontal_distance..=horizontal_distance {
                for z in -horizontal_distance..=horizontal_distance {
                    let chunk_pos = ChunkPos::new(
                        camera_chunk_pos.x + x,
                        camera_chunk_pos.y + y,
                        camera_chunk_pos.z + z,
                    );

                    let distance_sq = x * x + y * y * 4 + z * z;
                    if distance_sq <= horizontal_distance * horizontal_distance {
                        chunks_to_keep.insert(chunk_pos);
                        if !self.chunks.lock().contains_key(&chunk_pos) {
                            self.chunk_worker.queue_chunk_generation(chunk_pos);
                        }
                    }
                }
            }
        }

        self.chunks.lock().retain(|pos, _| {
            let dx = pos.x - camera_chunk_pos.x;
            let dy = pos.y - camera_chunk_pos.y;
            let dz = pos.z - camera_chunk_pos.z;
            dx * dx + dy * dy * 4 + dz * dz <= (horizontal_distance + 2) * (horizontal_distance + 2)
        });
    }
}
