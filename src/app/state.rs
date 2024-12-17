use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

use super::window;

pub struct AppState {
    window: Option<Window>,
}

impl AppState {
    pub fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = window::create_window(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing application");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                println!("Window resized to {}x{}", size.width, size.height);
                // Potentially trigger renderer resize
            }
            WindowEvent::RedrawRequested => {
                // Render logic will go here later
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
