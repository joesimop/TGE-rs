use winit::event_loop::{ControlFlow, EventLoop};

mod window;
mod state;

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut app_state = state::AppState::new();

    event_loop.run_app(&mut app_state).unwrap();
}
