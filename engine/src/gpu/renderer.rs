use super::{InstanceRaw, Vertex};
use crate::editor::Editor;
use winit::window::Window;

pub fn execute_render_pass(
    surface: &wgpu::Surface<'static>,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    config: &wgpu::SurfaceConfiguration,
    renderer: &Renderer,
    vertex_buffer: &wgpu::Buffer,
    instance_buffer: &wgpu::Buffer,
    editor: &mut Editor,
    window: &Window,
) -> Result<(), String> {
    let surface_texture = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame) => frame,
        wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
            return Ok(());
        }
        wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
            surface.configure(device, config);
            return Ok(());
        }
        wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
    };

    let view = surface_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let full_output = editor.draw_ui(window);
    let egui::FullOutput {
        shapes,
        pixels_per_point,
        mut textures_delta,
        ..
    } = full_output;
    let clipped_primitives = editor.egui_ctx.tessellate(shapes, pixels_per_point);

    for (id, image_delta) in &textures_delta.set {
        for delta in image_delta {
            editor
                .egui_renderer
                .update_texture(device, queue, *id, delta);
        }
    }

    let screen_descriptor = egui_wgpu::ScreenDescriptor {
        size_in_pixels: [config.width, config.height],
        pixels_per_point: window.scale_factor() as f32,
    };

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    editor.egui_renderer.update_buffers(
        device,
        queue,
        &mut encoder,
        &clipped_primitives,
        &screen_descriptor,
    );

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        render_pass.set_pipeline(&renderer.render_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.draw(0..3, 0..1);

        editor.egui_renderer.render(
            &mut render_pass.forget_lifetime(),
            &clipped_primitives,
            &screen_descriptor,
        );
    }

    for id in &textures_delta.free {
        editor.egui_renderer.free_texture(id);
    }

    queue.submit(std::iter::once(encoder.finish()));
    queue.present(surface_texture);

    textures_delta.clear();

    Ok(())
}

pub struct Renderer {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Model Matrix Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("model_matrix.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc()), Some(InstanceRaw::desc())],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self { render_pipeline }
    }
}
