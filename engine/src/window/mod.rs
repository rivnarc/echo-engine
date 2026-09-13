use crate::errors;
use crate::gpu_context::GpuContext;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            gpu: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Echo Engine")
            .with_inner_size(LogicalSize::new(800.0, 600.0));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let gpu = GpuContext::new(window.clone());

        self.window = Some(window);
        self.gpu = Some(gpu);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(new_size);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(gpu) = &mut self.gpu {
                    match gpu.render() {
                        Ok(_) => {}
                        Err(_e) => errors::set_error(0.21),
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
