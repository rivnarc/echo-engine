use crate::errors;
use std::sync::Arc;
use winit::window::Window;

pub struct InitializedGpu {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub fn create_gpu_resources(window: Arc<Window>) -> InitializedGpu {
    let size = window.inner_size();

    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(window).unwrap();

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
        ..Default::default()
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: Default::default(),
        ..Default::default()
    }))
    .unwrap();

    let config = match surface.get_default_config(&adapter, size.width.max(1), size.height.max(1)) {
        Some(conf) => conf,
        None => {
            errors::set_error(3.1);
            panic!("Failed to get default surface configuration");
        }
    };

    surface.configure(&device, &config);

    InitializedGpu {
        surface,
        device,
        queue,
        config,
    }
}
