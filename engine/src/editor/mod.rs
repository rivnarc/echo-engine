use glam::Vec2;

pub struct EditorState {
    pub position: Vec2,
    pub rotation_deg: f32,
    pub scale: Vec2,
    pub color: [f32; 4],
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation_deg: 0.0,
            scale: Vec2::ONE,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

pub struct Editor {
    pub state: EditorState,
    pub egui_ctx: egui::Context,
    pub egui_winit: egui_winit::State,
    pub egui_renderer: egui_wgpu::Renderer,
}

impl Editor {
    pub fn new(
        window: &winit::window::Window,
        device: &wgpu::Device,
        output_format: wgpu::TextureFormat,
    ) -> Self {
        let egui_ctx = egui::Context::default();

        let egui_winit = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let egui_renderer =
            egui_wgpu::Renderer::new(device, output_format, egui_wgpu::RendererOptions::default());

        Self {
            state: EditorState::default(),
            egui_ctx,
            egui_winit,
            egui_renderer,
        }
    }

    pub fn draw_ui(&mut self, window: &winit::window::Window) -> egui::FullOutput {
        let raw_input = self.egui_winit.take_egui_input(window);

        self.egui_ctx.run_ui(raw_input, |ctx| {
            egui::Window::new("2D Engine Editor").show(ctx, |ui| {
                ui.heading("Transform");

                ui.label("Position:");
                ui.add(egui::Slider::new(&mut self.state.position.x, -1.0..=1.0).text("X"));
                ui.add(egui::Slider::new(&mut self.state.position.y, -1.0..=1.0).text("Y"));

                ui.separator();
                ui.label("Rotation & Scale:");
                ui.add(
                    egui::Slider::new(&mut self.state.rotation_deg, 0.0..=360.0)
                        .text("Angle (deg)"),
                );
                ui.add(egui::Slider::new(&mut self.state.scale.x, 0.1..=3.0).text("Scale X"));
                ui.add(egui::Slider::new(&mut self.state.scale.y, 0.1..=3.0).text("Scale Y"));

                ui.separator();
                ui.heading("Material");
                ui.color_edit_button_rgba_unmultiplied(&mut self.state.color);
            });
        })
    }
}
