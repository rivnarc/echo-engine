pub mod init;
pub mod renderer;

use std::sync::Arc;
use winit::window::Window;

pub struct GpuContext {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl GpuContext {
    pub fn new(window: Arc<Window>) -> Self {
        let init_data = init::create_gpu_resources(window);
        Self {
            surface: init_data.surface,
            device: init_data.device,
            queue: init_data.queue,
            config: init_data.config,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), String> {
        renderer::execute_render_pass(&self.surface, &self.device, &self.queue, &self.config)
    }
}
