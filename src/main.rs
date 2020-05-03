mod components;
mod input;
mod math;
mod render;
mod utils;

extern crate gl;
extern crate glutin;
extern crate image;

use crate::input::InputHandler;
use crate::render::camera::PerspectiveCamera;
use crate::render::renderer::Renderer;
use crate::render::Display;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use std::time::Instant;

const FPS_REFRESH_TIMEOUT: u64 = 1;

fn main() {
    let event_loop = EventLoop::new();

    let display = Display::create("sick opengl shitshow", &event_loop);
    let size = display.context.window().inner_size();
    let aspect_ratio = size.width as f32 / size.height as f32;

    let mut renderer = Renderer::init(size.width, size.height);
    let mut camera = PerspectiveCamera::new(70.0, 0.1, 1024.0, aspect_ratio);
    let mut input_handler = InputHandler::default();

    let mut fps: u32 = 0;
    let mut last_time = Instant::now();
    let mut last_fps_update = Instant::now();

    // TODO: remove this temporary data
    use crate::render::models::Cube;
    let cube = Cube::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::LoopDestroyed => return,
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => {
                camera.set_aspect_ratio(size.width as f32 / size.height as f32);
                renderer.set_size(size.width, size.height);
            }
            WindowEvent::KeyboardInput { input, .. } => input_handler.process_keyboard(input),
            WindowEvent::CursorMoved { position, .. } => input_handler.process_cursor(position),
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => (),
        },
        Event::RedrawRequested(_) => {
            renderer.draw(&display, &camera, &cube);
        }
        Event::MainEventsCleared => {
            let time_delta = last_time.elapsed().as_secs_f32();
            last_time = Instant::now();

            if last_fps_update.elapsed().as_secs() >= FPS_REFRESH_TIMEOUT {
                fps = (1.0 / time_delta) as u32;
                println!("FPS: {}", fps);
                last_fps_update = Instant::now();
            }

            // should be a loop to updage every component instead of just the camera
            camera.update(&input_handler, &time_delta);
            renderer.update(&input_handler);
            input_handler.clear_cursor_delta();
            display.context.window().request_redraw();
        }
        _ => (),
    });
}
