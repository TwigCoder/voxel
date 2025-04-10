mod engine;
mod utils;
mod world;

use engine::state::State;
use std::time::{Duration, Instant};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Voxel Engine")
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(async { State::new(&window).await });

    let mut frame_count = 0;
    let mut fps = 0.0;
    let mut last_fps_update = Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            frame_count += 1;
            let now = Instant::now();

            if now.duration_since(last_fps_update) >= Duration::from_secs(1) {
                fps = frame_count as f64 / now.duration_since(last_fps_update).as_secs_f64();
                frame_count = 0;
                last_fps_update = now;

                window.set_title(&format!("Rust Voxel Engine | FPS: {:.1}", fps));
            }

            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
