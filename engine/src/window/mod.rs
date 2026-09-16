use crate::editor::Editor;
use crate::gpu::GpuContext;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub gpu: Option<GpuContext>,
    pub editor: Option<Editor>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes().with_title("2D Engine");
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let gpu = GpuContext::new(window.clone());
            let editor = Editor::new(&window, &gpu.device, gpu.config.format);

            self.window = Some(window);
            self.gpu = Some(gpu);
            self.editor = Some(editor);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let (window, gpu, editor) = match (&self.window, &mut self.gpu, &mut self.editor) {
            (Some(w), Some(g), Some(e)) => (w, g, e),
            _ => return,
        };

        let response = editor.egui_winit.on_window_event(window, &event);
        if response.consumed {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                gpu.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                let state = &editor.state;
                gpu.update_instance(
                    state.position,
                    state.rotation_deg.to_radians(),
                    state.scale,
                    glam::Vec4::from(state.color),
                );

                let _ = gpu.render(editor, window);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);
}
