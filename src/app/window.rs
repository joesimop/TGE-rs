use winit::event_loop::ActiveEventLoop;
use winit::window::{ Window, WindowAttributes };
use winit::dpi::LogicalSize;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub fn create_window(event_loop: &ActiveEventLoop) -> Option<Window> {
    let window_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    return Some(
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_inner_size(window_size)
                    .with_resizable(true)
            )
            .unwrap()
    );
}
