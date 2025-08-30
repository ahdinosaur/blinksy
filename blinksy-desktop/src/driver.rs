//! # Desktop Simulation Driver
//!
//! This module provides a graphical simulation of LED layouts and patterns for desktop development
//! and debugging. It implements the [`Driver`] trait, allowing it to be used as a drop-in
//! replacement for physical LED hardware.
//!
//! The simulator creates a 3D visualization window where:
//!
//! - LEDs are represented as small 3D objects
//! - LED positions match the layout's physical arrangement
//! - Colors and brightness updates are displayed in real-time
//!
//! ## Controls
//!
//! - **Mouse drag**: Rotate the camera around the LEDs
//! - **Mouse wheel**: Zoom in/out
//! - **R key**: Reset camera to default position
//! - **O key**: Toggle between orthographic and perspective projection
//!
//! ## Usage
//!
//! ```rust,no_run
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Shape2d, Vec2},
//!     patterns::rainbow::{Rainbow, RainbowParams}
//! };
//! use blinksy_desktop::{driver::Desktop, time::elapsed_in_ms};
//!
//! // Define your layout
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         horizontal_end: Vec2::new(1., -1.),
//!         vertical_end: Vec2::new(-1., 1.),
//!         horizontal_pixel_count: 16,
//!         vertical_pixel_count: 16,
//!         serpentine: true,
//!     }]
//! );
//!
//! // Create a control using the Desktop driver instead of physical hardware
//! let mut control = ControlBuilder::new_2d()
//!     .with_layout::<Layout>()
//!     .with_pattern::<Rainbow>(RainbowParams::default())
//!     .with_driver(Desktop::new_2d::<Layout>())
//!     .build();
//!
//! // Run your normal animation loop
//! loop {
//!     control.tick(elapsed_in_ms()).unwrap();
//!     std::thread::sleep(std::time::Duration::from_millis(16));
//! }
//! ```
//!
//! [`Driver`]: blinksy::driver::Driver

use blinksy::{
    color::{ColorCorrection, FromColor, LinearSrgb, Srgb},
    dimension::{Dim1d, Dim2d, Dim3d, LayoutForDim},
    driver::Driver,
    layout::{Layout1d, Layout2d, Layout3d},
};
use core::{fmt, marker::PhantomData};
use egui_miniquad as egui_mq;
use glam::{vec3, Mat4, Vec3, Vec4, Vec4Swizzles};
use miniquad::*;
use std::sync::mpsc::{channel, Receiver, SendError, Sender};

/// Configuration options for the desktop simulator.
///
/// Allows customizing the appearance and behavior of the LED simulator window.
#[derive(Clone, Debug)]
pub struct DesktopConfig {
    /// Window title
    pub window_title: String,

    /// Window width in pixels
    pub window_width: i32,

    /// Window height in pixels
    pub window_height: i32,

    /// Size of the LED representations
    pub led_radius: f32,

    /// Whether to use high DPI mode
    pub high_dpi: bool,

    /// Initial camera view mode (true for orthographic, false for perspective)
    pub orthographic_view: bool,

    /// Background color (R, G, B, A) where each component is 0.0 - 1.0
    pub background_color: (f32, f32, f32, f32),
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            window_title: "Blinksy".to_string(),
            window_width: 540,
            window_height: 540,
            led_radius: 0.05,
            high_dpi: true,
            orthographic_view: true,
            background_color: (0.1, 0.1, 0.1, 1.0),
        }
    }
}

/// Desktop driver for simulating LED layouts in a desktop window.
///
/// This struct implements the `Driver` trait and renders a visual
/// representation of your LED layout using miniquad.
///
/// # Type Parameters
///
/// * `Dim` - The dimension marker (Dim1d or Dim2d)
/// * `Layout` - The specific layout type
pub struct Desktop<Dim, Layout> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    brightness: f32,
    correction: ColorCorrection,
    sender: Sender<LedMessage>,
    is_window_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Desktop<Dim1d, ()> {
    /// Creates a new graphics driver for 1D layouts.
    ///
    /// This method initializes a rendering window showing a linear strip of LEDs.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout1d
    ///
    /// # Returns
    ///
    /// A Desktop driver configured for the specified 1D layout
    pub fn new_1d<Layout>() -> (Desktop<Dim1d, Layout>, DesktopStage)
    where
        Layout: Layout1d,
    {
        Self::new_1d_with_config::<Layout>(DesktopConfig::default())
    }

    /// Creates a new graphics driver for 1D layouts with custom configuration.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout1d
    ///
    /// # Parameters
    ///
    /// * `config` - Configuration options for the simulator window
    ///
    /// # Returns
    ///
    /// A Desktop driver configured for the specified 1D layout
    pub fn new_1d_with_config<Layout>(
        config: DesktopConfig,
    ) -> (Desktop<Dim1d, Layout>, DesktopStage)
    where
        Layout: Layout1d,
    {
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);
        for x in Layout::points() {
            positions.push(vec3(x, 0.0, 0.0));
        }

        let (sender, receiver) = channel();
        let is_window_closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_window_closed_2 = is_window_closed.clone();
        let stage = DesktopStage::new(positions, receiver, config, is_window_closed_2);

        (
            Desktop {
                dim: PhantomData,
                layout: PhantomData,
                brightness: 1.0,
                correction: ColorCorrection::default(),
                sender,
                is_window_closed,
            },
            stage,
        )
    }
}

impl Desktop<Dim2d, ()> {
    /// Creates a new graphics driver for 2D layouts.
    ///
    /// This method initializes a rendering window showing a 2D arrangement of LEDs
    /// based on the layout's coordinates.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout2d
    ///
    /// # Returns
    ///
    /// A Desktop driver configured for the specified 2D layout
    pub fn new_2d<Layout>() -> Desktop<Dim2d, Layout>
    where
        Layout: Layout2d,
    {
        Self::new_2d_with_config::<Layout>(DesktopConfig::default())
    }

    /// Creates a new graphics driver for 2D layouts with custom configuration.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout2d
    ///
    /// # Parameters
    ///
    /// * `config` - Configuration options for the simulator window
    ///
    /// # Returns
    ///
    /// A Desktop driver configured for the specified 2D layout
    pub fn new_2d_with_config<Layout>(config: DesktopConfig) -> Desktop<Dim2d, Layout>
    where
        Layout: Layout2d,
    {
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);
        for point in Layout::points() {
            positions.push(vec3(point.x, point.y, 0.0));
        }

        let (sender, receiver) = channel();
        let is_window_closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_window_closed_2 = is_window_closed.clone();

        std::thread::spawn(move || {
            DesktopStage::start(move || {
                DesktopStage::new(positions, receiver, config, is_window_closed_2)
            });
        });

        Desktop {
            dim: PhantomData,
            layout: PhantomData,
            brightness: 1.0,
            correction: ColorCorrection::default(),
            sender,
            is_window_closed,
        }
    }
}

impl Desktop<Dim3d, ()> {
    /// Creates a new graphics driver for 3D layouts.
    ///
    /// This method initializes a rendering window showing a 3D arrangement of LEDs
    /// based on the layout's coordinates.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout3d
    ///
    /// # Returns
    ///
    /// A Desktop driver configured for the specified 3D layout
    pub fn new_3d<Layout>() -> Desktop<Dim3d, Layout>
    where
        Layout: Layout3d,
    {
        Self::new_3d_with_config::<Layout>(DesktopConfig::default())
    }

    /// Creates a new graphics driver for 3D layouts with custom configuration.
    ///
    /// # Type Parameters
    ///
    /// * `Layout` - The layout type implementing Layout3d
    ///
    /// # Parameters
    ///
    /// * `config` - Configuration options for the simulator window
    ///
    /// # Returns
    ///
    /// A Desktop driver configured for the specified 3D layout
    pub fn new_3d_with_config<Layout>(config: DesktopConfig) -> Desktop<Dim3d, Layout>
    where
        Layout: Layout3d,
    {
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);
        for point in Layout::points() {
            positions.push(vec3(point.x, point.y, point.z));
        }

        let (sender, receiver) = channel();
        let is_window_closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_window_closed_2 = is_window_closed.clone();

        std::thread::spawn(move || {
            DesktopStage::start(move || {
                DesktopStage::new(positions, receiver, config, is_window_closed_2)
            });
        });

        Desktop {
            dim: PhantomData,
            layout: PhantomData,
            brightness: 1.0,
            correction: ColorCorrection::default(),
            sender,
            is_window_closed,
        }
    }
}

impl<Dim, Layout> Desktop<Dim, Layout> {
    fn send(&self, message: LedMessage) -> Result<(), DesktopError> {
        if self
            .is_window_closed
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            return Err(DesktopError::WindowClosed);
        }
        self.sender.send(message)?;
        Ok(())
    }
}

/// Errors that can occur when using the Desktop driver.
#[derive(Debug)]
pub enum DesktopError {
    /// Sending to the render thread failed because it has already hung up.
    ChannelSend,

    /// Window has been closed.
    WindowClosed,
}

impl fmt::Display for DesktopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DesktopError::ChannelSend => write!(f, "render thread channel disconnected"),
            DesktopError::WindowClosed => write!(f, "window closed"),
        }
    }
}

impl core::error::Error for DesktopError {}

impl From<SendError<LedMessage>> for DesktopError {
    fn from(_: SendError<LedMessage>) -> Self {
        DesktopError::ChannelSend
    }
}

/// Messages for communication with the rendering thread.
enum LedMessage {
    /// Update the colors of all LEDs
    UpdateColors(Vec<LinearSrgb>),

    /// Update the global brightness
    UpdateBrightness(f32),

    /// Update the global color correction
    UpdateColorCorrection(ColorCorrection),

    /// Terminate the rendering thread
    Quit,
}

impl<Dim, Layout> Driver for Desktop<Dim, Layout>
where
    Layout: LayoutForDim<Dim>,
{
    type Error = DesktopError;
    type Color = LinearSrgb;

    fn write<I, C>(
        &mut self,
        pixels: I,
        brightness: f32,
        correction: ColorCorrection,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        if self.brightness != brightness {
            self.brightness = brightness;
            self.send(LedMessage::UpdateBrightness(brightness))?;
        }

        if self.correction != correction {
            self.correction = correction;
            self.send(LedMessage::UpdateColorCorrection(correction))?;
        }

        let colors: Vec<LinearSrgb> = pixels
            .into_iter()
            .map(|color| LinearSrgb::from_color(color))
            .collect();

        self.send(LedMessage::UpdateColors(colors))?;
        Ok(())
    }
}

impl<Dim, Layout> Drop for Desktop<Dim, Layout> {
    fn drop(&mut self) {
        let _ = self.send(LedMessage::Quit);
    }
}

/// Camera controller for the 3D LED visualization.
///
/// Handles camera movement, rotation, and projection calculations.
struct Camera {
    /// Distance from camera to target
    distance: f32,

    /// Position camera is looking at
    target: Vec3,

    /// Horizontal rotation angle in radians
    yaw: f32,

    /// Vertical rotation angle in radians
    pitch: f32,

    /// Width/height ratio of the viewport
    aspect_ratio: f32,

    /// Use orthographic (true) or perspective (false) projection
    use_orthographic: bool,

    /// Field of view in radians (used for perspective projection)
    fov: f32,
}

impl Camera {
    const DEFAULT_DISTANCE: f32 = 2.0;
    const DEFAULT_TARGET: Vec3 = Vec3::ZERO;
    const DEFAULT_YAW: f32 = core::f32::consts::PI * 0.5;
    const DEFAULT_PITCH: f32 = 0.0;
    const MIN_DISTANCE: f32 = 0.5;
    const MAX_DISTANCE: f32 = 10.0;
    const MAX_PITCH: f32 = core::f32::consts::PI / 2.0 - 0.1;
    const MIN_PITCH: f32 = -core::f32::consts::PI / 2.0 + 0.1;

    /// Create a new camera with default settings
    fn new(aspect_ratio: f32, use_orthographic: bool) -> Self {
        let default_fov = 2.0 * ((1.0 / Self::DEFAULT_DISTANCE).atan());
        Self {
            distance: Self::DEFAULT_DISTANCE,
            target: Self::DEFAULT_TARGET,
            yaw: Self::DEFAULT_YAW,
            pitch: Self::DEFAULT_PITCH,
            aspect_ratio,
            use_orthographic,
            fov: default_fov,
        }
    }

    /// Reset camera to default position and orientation
    fn reset(&mut self) {
        self.distance = Self::DEFAULT_DISTANCE;
        self.target = Self::DEFAULT_TARGET;
        self.yaw = Self::DEFAULT_YAW;
        self.pitch = Self::DEFAULT_PITCH;
    }

    /// Update camera aspect ratio when window is resized
    fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    /// Toggle between orthographic and perspective projection
    fn toggle_projection_mode(&mut self) {
        self.use_orthographic = !self.use_orthographic;
    }

    /// Update camera rotation based on mouse movement
    fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw -= delta_x * 0.01;
        self.pitch += delta_y * 0.01;
        self.pitch = self.pitch.clamp(Self::MIN_PITCH, Self::MAX_PITCH);
    }

    /// Update camera zoom based on mouse wheel movement
    fn zoom(&mut self, delta: f32) {
        self.distance -= delta * 0.2;
        self.distance = self.distance.clamp(Self::MIN_DISTANCE, Self::MAX_DISTANCE);
    }

    /// Calculate the current camera position based on spherical coordinates
    fn position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.sin();
        self.target + vec3(x, y, z)
    }

    /// Calculate view matrix for the current camera state
    fn view_matrix(&self) -> Mat4 {
        let eye = self.position();
        let up = if self.pitch.abs() > std::f32::consts::PI * 0.49 {
            Vec3::new(self.yaw.sin(), 0.0, -self.yaw.cos())
        } else {
            Vec3::Y
        };
        Mat4::look_at_rh(eye, self.target, up)
    }

    /// Calculate projection matrix based on current settings
    fn projection_matrix(&self) -> Mat4 {
        if self.use_orthographic {
            let vertical_size = 1.0 * (self.distance / 2.0);
            Mat4::orthographic_rh_gl(
                -vertical_size * self.aspect_ratio,
                vertical_size * self.aspect_ratio,
                -vertical_size,
                vertical_size,
                -100.0,
                100.0,
            )
        } else {
            Mat4::perspective_rh_gl(self.fov, self.aspect_ratio, 0.1, 100.0)
        }
    }

    /// Get the combined view-projection matrix
    fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

/// Manages LED selection and interaction
struct LedPicker {
    positions: Vec<Vec3>,
    selected_led: Option<usize>,
    radius: f32,
}

impl LedPicker {
    fn new(positions: Vec<Vec3>, radius: f32) -> Self {
        Self {
            positions,
            selected_led: None,
            radius,
        }
    }

    /// Convert screen coordinates to a ray in world space
    fn screen_pos_to_ray(&self, screen_x: f32, screen_y: f32, camera: &Camera) -> (Vec3, Vec3) {
        let (width, height) = window::screen_size();

        // Normalize device coordinates (-1 to 1)
        let x = 2.0 * screen_x / width - 1.0;
        let y = 1.0 - 2.0 * screen_y / height;

        // Compute inverse matrices
        let proj_inv = camera.projection_matrix().inverse();
        let view_inv = camera.view_matrix().inverse();

        // Calculate ray origin and direction
        let near_point = proj_inv * Vec4::new(x, y, -1.0, 1.0);
        let far_point = proj_inv * Vec4::new(x, y, 1.0, 1.0);

        let near_point = near_point / near_point.w;
        let far_point = far_point / far_point.w;

        let near_point_world = view_inv * near_point;
        let far_point_world = view_inv * far_point;

        let origin = near_point_world.xyz();
        let direction = (far_point_world.xyz() - near_point_world.xyz()).normalize();

        (origin, direction)
    }

    /// Pick an LED based on screen coordinates
    fn pick_led(&self, screen_x: f32, screen_y: f32, camera: &Camera) -> Option<usize> {
        let (ray_origin, ray_direction) = self.screen_pos_to_ray(screen_x, screen_y, camera);

        // Find the closest LED that intersects with the ray
        let mut closest_led = None;
        let mut closest_distance = f32::MAX;

        for (i, &position) in self.positions.iter().enumerate() {
            // Sphere-ray intersection test
            let oc = ray_origin - position;
            let a = ray_direction.dot(ray_direction);
            let b = 2.0 * oc.dot(ray_direction);
            let c = oc.dot(oc) - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant > 0.0 {
                let t = (-b - discriminant.sqrt()) / (2.0 * a);
                if t > 0.0 && t < closest_distance {
                    closest_distance = t;
                    closest_led = Some(i);
                }
            }
        }

        closest_led
    }

    /// Try to select an LED at the given screen coordinates
    fn try_select_at(&mut self, screen_x: f32, screen_y: f32, camera: &Camera) {
        self.selected_led = self.pick_led(screen_x, screen_y, camera);
    }

    /// Clear the current selection
    fn clear_selection(&mut self) {
        self.selected_led = None;
    }
}

/// Manages UI state and rendering
struct UiManager {
    egui_mq: egui_mq::EguiMq,
    want_mouse_capture: bool,
}

impl UiManager {
    fn new(ctx: &mut dyn RenderingBackend) -> Self {
        Self {
            egui_mq: egui_mq::EguiMq::new(ctx),
            want_mouse_capture: false,
        }
    }

    /// Forward mouse motion events to egui
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    /// Forward mouse wheel events to egui
    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_wheel_event(x, y);
    }

    /// Forward mouse button down events to egui
    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(button, x, y);
    }

    /// Forward mouse button up events to egui
    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(button, x, y);
    }

    /// Forward key down events to egui
    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    /// Forward key up events to egui
    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }

    /// Forward character events to egui
    fn char_event(&mut self, character: char) {
        self.egui_mq.char_event(character);
    }

    /// Render the LED information UI
    #[allow(clippy::too_many_arguments)]
    fn render_led_info(
        &mut self,
        ctx: &mut dyn RenderingBackend,
        led_picker: &mut LedPicker,
        positions: &[Vec3],
        colors: &[LinearSrgb],
        brightness: f32,
        correction: ColorCorrection,
    ) {
        self.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
            self.want_mouse_capture = egui_ctx.wants_pointer_input();

            // Only show LED info window if an LED is selected
            if let Some(led_idx) = led_picker.selected_led {
                let pos = positions[led_idx];
                let color = colors[led_idx];

                let (red, green, blue) = (color.red, color.green, color.blue);

                // Apply brightness
                let (bright_red, bright_green, bright_blue) =
                    (red * brightness, green * brightness, blue * brightness);

                // Apply color correction
                let (correct_red, correct_green, correct_blue) = (
                    bright_red * correction.red,
                    bright_green * correction.green,
                    bright_blue * correction.blue,
                );

                // Convert to sRGB
                let Srgb {
                    red: srgb_red,
                    green: srgb_green,
                    blue: srgb_blue,
                } = LinearSrgb::new(correct_red, correct_green, correct_blue).to_srgb();

                egui::Window::new("LED Information")
                    .collapsible(false)
                    .resizable(false)
                    .show(egui_ctx, |ui| {
                        ui.label(format!("LED Index: {}", led_idx));
                        ui.label(format!(
                            "Position: ({:.3}, {:.3}, {:.3})",
                            pos.x, pos.y, pos.z
                        ));

                        // Display raw RGB values
                        ui.label(format!(
                            "Linear RGB: R={:.3}, G={:.3}, B={:.3}",
                            red, green, blue,
                        ));

                        // Display global brightness
                        ui.label(format!("Global Brightness: {:.3}", brightness));

                        // Display brightness-adjusted RGB values
                        ui.label(format!(
                            "Brightness-adjusted RGB: R={:.3}, G={:.3}, B={:.3}",
                            bright_red, bright_green, bright_blue
                        ));

                        // Display global color correction
                        ui.label(format!(
                            "Global Color Correction: R={:.3}, G={:.3}, B={:.3}",
                            correction.red, correction.green, correction.blue
                        ));

                        // Display brightness-adjusted RGB values
                        ui.label(format!(
                            "Correction-adjusted RGB: R={:.3}, G={:.3}, B={:.3}",
                            correct_red, correct_green, correct_blue
                        ));

                        // Display sRGB values
                        ui.label(format!(
                            "Final sRGB: R={:.3}, G={:.3}, B={:.3}",
                            srgb_red, srgb_green, srgb_blue
                        ));

                        // Show color preview
                        let (_, color_rect) =
                            ui.allocate_space(egui::vec2(ui.available_width(), 30.0));
                        let color_preview = egui::Color32::from_rgb(
                            (srgb_red * 255.0) as u8,
                            (srgb_green * 255.0) as u8,
                            (srgb_blue * 255.0) as u8,
                        );
                        ui.painter().rect_filled(color_rect, 4.0, color_preview);
                        ui.add_space(10.0); // Space after the color preview

                        // Deselect button
                        if ui.button("Deselect").clicked() {
                            led_picker.selected_led = None;
                        }
                    });
            }
        });
    }

    /// Draw egui content
    fn draw(&mut self, ctx: &mut dyn RenderingBackend) {
        self.egui_mq.draw(ctx);
    }
}

/// Manages rendering of LEDs
struct Renderer {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Renderer {
    fn new(ctx: &mut dyn RenderingBackend, led_radius: f32) -> Self {
        let vertex_buffer = Self::create_vertex_buffer(ctx, led_radius);
        let index_buffer = Self::create_index_buffer(ctx);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_pos", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_color", VertexFormat::Float4, 0),
                VertexAttribute::with_buffer("in_inst_pos", VertexFormat::Float3, 1),
                VertexAttribute::with_buffer("in_inst_color", VertexFormat::Float4, 2),
            ],
            shader,
            PipelineParams {
                depth_test: Comparison::LessOrEqual,
                depth_write: true,
                ..Default::default()
            },
        );

        Self { pipeline, bindings }
    }

    fn create_vertex_buffer(ctx: &mut dyn RenderingBackend, r: f32) -> BufferId {
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            0.0, -r, 0.0, 1.0, 0.0, 0.0, 1.0,
            r, 0.0, r, 0.0, 1.0, 0.0, 1.0,
            r, 0.0, -r, 0.0, 0.0, 1.0, 1.0,
            -r, 0.0, -r, 1.0, 1.0, 0.0, 1.0,
            -r, 0.0, r, 0.0, 1.0, 1.0, 1.0,
            0.0, r, 0.0, 1.0, 0.0, 1.0, 1.0,
        ];

        ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(vertices),
        )
    }

    fn create_index_buffer(ctx: &mut dyn RenderingBackend) -> BufferId {
        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1,
            5, 1, 2, 5, 2, 3, 5, 3, 4, 5, 4, 1
        ];

        ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(indices),
        )
    }

    fn update_positions_buffer(
        &mut self,
        ctx: &mut dyn RenderingBackend,
        positions: &[Vec3],
    ) -> BufferId {
        let positions_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(positions),
        );
        self.bindings.vertex_buffers.push(positions_buffer);
        positions_buffer
    }

    fn update_colors_buffer(
        &mut self,
        ctx: &mut dyn RenderingBackend,
        colors: &[Vec4],
    ) -> BufferId {
        let colors_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(colors),
        );
        self.bindings.vertex_buffers.push(colors_buffer);
        colors_buffer
    }

    fn render(
        &self,
        ctx: &mut dyn RenderingBackend,
        positions: &[Vec3],
        view_proj: Mat4,
        background_color: (f32, f32, f32, f32),
    ) {
        let (r, g, b, a) = background_color;

        // Clear the background
        ctx.begin_default_pass(PassAction::clear_color(r, g, b, a));

        // Draw the LEDs
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms { mvp: view_proj }));

        ctx.draw(0, 24, positions.len() as i32);
        ctx.end_render_pass();
    }
}

/// The rendering stage that handles the miniquad window and OpenGL drawing.
pub struct DesktopStage {
    ctx: Box<dyn RenderingBackend>,
    positions: Vec<Vec3>,
    colors: Vec<LinearSrgb>,
    colors_buffer: Vec<Vec4>,
    brightness: f32,
    correction: ColorCorrection,
    receiver: Receiver<LedMessage>,
    camera: Camera,
    config: DesktopConfig,
    is_window_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    mouse_down: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    ui_manager: UiManager,
    led_picker: LedPicker,
    renderer: Renderer,
}

impl DesktopStage {
    /// Start the rendering loop.
    pub fn start<F, H>(f: F)
    where
        F: 'static + FnOnce() -> H,
        H: EventHandler + 'static,
    {
        let conf = conf::Conf {
            window_title: "Blinksy".to_string(),
            window_width: 800,
            window_height: 600,
            high_dpi: true,
            ..Default::default()
        };
        miniquad::start(conf, move || Box::new(f()));
    }

    /// Create a new DesktopStage with the given LED positions, colors, and configuration.
    fn new(
        positions: Vec<Vec3>,
        receiver: Receiver<LedMessage>,
        config: DesktopConfig,
        is_window_closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        // Initialize UI manager
        let ui_manager = UiManager::new(&mut *ctx);

        // Initialize LED picker
        let led_picker = LedPicker::new(positions.clone(), config.led_radius);

        // Initialize renderer
        let renderer = Renderer::new(&mut *ctx, config.led_radius);

        // Initialize camera
        let (width, height) = window::screen_size();
        let camera = Camera::new(width / height, config.orthographic_view);

        // Initialize colors buffer
        let colors_buffer = (0..positions.len())
            .map(|_| Vec4::new(0.0, 0.0, 0.0, 1.0))
            .collect();

        // Create the stage
        let mut stage = Self {
            ctx,
            positions: positions.clone(),
            colors: Vec::new(),
            colors_buffer,
            brightness: 1.0,
            correction: ColorCorrection::default(),
            receiver,
            camera,
            config,
            is_window_closed,
            mouse_down: false,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
            ui_manager,
            led_picker,
            renderer,
        };

        // Setup buffers
        stage
            .renderer
            .update_positions_buffer(&mut *stage.ctx, &positions);
        stage
            .renderer
            .update_colors_buffer(&mut *stage.ctx, &stage.colors_buffer);

        stage
    }

    /// Process any pending messages from the main thread.
    fn process_messages(&mut self) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                LedMessage::UpdateColors(colors) => {
                    self.colors = colors;
                }
                LedMessage::UpdateBrightness(brightness) => {
                    self.brightness = brightness;
                }
                LedMessage::UpdateColorCorrection(correction) => {
                    self.correction = correction;
                }
                LedMessage::Quit => {
                    window::quit();
                }
            }
        }
    }

    /// Handles input for camera controls
    fn handle_camera_input(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::R => {
                self.camera.reset();
            }
            KeyCode::O => {
                self.camera.toggle_projection_mode();
            }
            KeyCode::Escape => {
                // Clear selection when Escape is pressed
                self.led_picker.clear_selection();
            }
            _ => {}
        }
    }
}

impl EventHandler for DesktopStage {
    fn update(&mut self) {
        self.process_messages();
    }

    fn draw(&mut self) {
        let colors_buffer: Vec<Vec4> = self
            .colors
            .iter()
            .map(|color| {
                let (red, green, blue) = (color.red, color.green, color.blue);

                // Apply brightness
                let (red, green, blue) = (
                    red * self.brightness,
                    green * self.brightness,
                    blue * self.brightness,
                );

                // Apply color correction
                let (red, green, blue) = (
                    red * self.correction.red,
                    green * self.correction.green,
                    blue * self.correction.blue,
                );

                // Convert to sRGB
                let Srgb { red, green, blue } = LinearSrgb::new(red, green, blue).to_srgb();

                Vec4::new(red, green, blue, 1.)
            })
            .collect();

        // Update colors buffer
        self.colors_buffer = colors_buffer;
        self.ctx.buffer_update(
            self.renderer.bindings.vertex_buffers[2],
            BufferSource::slice(&self.colors_buffer),
        );

        // Render the LEDs
        let view_proj = self.camera.view_projection_matrix();
        self.renderer.render(
            &mut *self.ctx,
            &self.positions,
            view_proj,
            self.config.background_color,
        );

        // Render UI with LED info if needed
        self.ui_manager.render_led_info(
            &mut *self.ctx,
            &mut self.led_picker,
            &self.positions,
            &self.colors,
            self.brightness,
            self.correction,
        );

        // Draw egui
        self.ui_manager.draw(&mut *self.ctx);

        self.ctx.commit_frame();
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.camera.set_aspect_ratio(width / height);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.ui_manager.mouse_motion_event(x, y);

        if self.mouse_down && !self.ui_manager.want_mouse_capture {
            let dx = x - self.last_mouse_x;
            let dy = y - self.last_mouse_y;
            self.camera.rotate(dx, dy);
        }
        self.last_mouse_x = x;
        self.last_mouse_y = y;
    }

    fn mouse_wheel_event(&mut self, x: f32, y: f32) {
        self.ui_manager.mouse_wheel_event(x, y);

        if !self.ui_manager.want_mouse_capture {
            self.camera.zoom(y);
        }
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.ui_manager.mouse_button_down_event(button, x, y);

        if button == MouseButton::Left && !self.ui_manager.want_mouse_capture {
            // Check for LED selection on click
            if !self.mouse_down {
                // Only do picking when button is first pressed
                self.led_picker.try_select_at(x, y, &self.camera);
            }

            self.mouse_down = true;
            self.last_mouse_x = x;
            self.last_mouse_y = y;
        }
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        self.ui_manager.mouse_button_up_event(button, x, y);

        if button == MouseButton::Left {
            self.mouse_down = false;
        }
    }

    fn key_down_event(&mut self, keycode: KeyCode, keymods: KeyMods, _repeat: bool) {
        self.ui_manager.key_down_event(keycode, keymods);

        if !self.ui_manager.want_mouse_capture {
            self.handle_camera_input(keycode);
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, keymods: KeyMods) {
        self.ui_manager.key_up_event(keycode, keymods);
    }

    fn char_event(&mut self, character: char, _keymods: KeyMods, _repeat: bool) {
        self.ui_manager.char_event(character);
    }

    fn quit_requested_event(&mut self) {
        self.is_window_closed
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Shader definitions for rendering LEDs
mod shader {
    use miniquad::*;

    /// Vertex shader for LED rendering
    pub const VERTEX: &str = r#"#version 100
    attribute vec3 in_pos;
    attribute vec4 in_color;
    attribute vec3 in_inst_pos;
    attribute vec4 in_inst_color;

    varying lowp vec4 color;

    uniform mat4 mvp;

    void main() {
        vec4 pos = vec4(in_pos + in_inst_pos, 1.0);
        gl_Position = mvp * pos;
        color = in_inst_color;
    }
    "#;

    /// Fragment shader for LED rendering
    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }
    "#;

    /// Shader metadata describing uniforms
    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    /// Uniform structure for shader
    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}
