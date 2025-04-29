//! # Desktop Desktop Simulation
//!
//! This module provides a graphical simulation of LED layouts and patterns for desktop development
//! and debugging. It implements the `LedDriver` trait, allowing it to be used as a drop-in
//! replacement for physical LED hardware.
//!
//! The simulator creates a 3D visualization window where:
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
//! ```rust
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Shape2d, Vec2},
//!     patterns::{Rainbow, RainbowParams}
//! };
//! use blinksy_desktop::{drivers::Desktop, time::elapsed},
//!
//! // Define your layout
//! layout2d!(
//!     Layout,
//!     [Shape2d::Grid {
//!         start: Vec2::new(-1., -1.),
//!         row_end: Vec2::new(1., -1.),
//!         col_end: Vec2::new(-1., 1.),
//!         row_pixel_count: 16,
//!         col_pixel_count: 16,
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
//!     control.tick(elapsed()).unwrap();
//!     std::thread::sleep(std::time::Duration::from_millis(16));
//! }
//! ```

use blinksy::{
    color::{FromColor, LinSrgb, Srgb},
    dimension::{Dim1d, Dim2d, LayoutForDim},
    driver::LedDriver,
    layout::{Layout1d, Layout2d},
};
use core::{fmt, marker::PhantomData};
use glam::{vec3, Mat4, Vec3, Vec4};
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
/// This struct implements the `LedDriver` trait and renders a visual
/// representation of your LED layout using miniquad.
///
/// # Type Parameters
///
/// * `Dim` - The dimension marker (Dim1d or Dim2d)
/// * `Layout` - The specific layout type
pub struct Desktop<Dim, Layout> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    sender: Sender<LedMessage>,
    brightness: f32,
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
    pub fn new_1d<Layout>() -> Desktop<Dim1d, Layout>
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
    pub fn new_1d_with_config<Layout>(config: DesktopConfig) -> Desktop<Dim1d, Layout>
    where
        Layout: Layout1d,
    {
        let (sender, receiver) = channel();

        // Calculate all LED positions for a 1D layout
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);

        // Create a horizontal strip of LEDs from -1.0 to 1.0
        let spacing = if Layout::PIXEL_COUNT > 1 {
            2.0 / (Layout::PIXEL_COUNT as f32 - 1.0)
        } else {
            0.0
        };

        for i in 0..Layout::PIXEL_COUNT {
            let x = -1.0 + (i as f32 * spacing);
            positions.push(vec3(x, 0.0, 0.0));
        }

        // Create initial black colors
        let colors = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); Layout::PIXEL_COUNT];

        // Start rendering thread
        std::thread::spawn(move || {
            DesktopStage::start(|| DesktopStage::new(positions, colors, receiver, config));
        });

        Desktop {
            dim: PhantomData,
            layout: PhantomData,
            sender,
            brightness: 1.0,
        }
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
        let (sender, receiver) = channel();

        // Calculate all LED positions for a 2D layout
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);

        // Convert layout points to 3D positions
        for point in Layout::points() {
            positions.push(vec3(point.x, point.y, 0.0));
        }

        // Create initial black colors
        let colors = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); Layout::PIXEL_COUNT];

        // Start rendering thread
        std::thread::spawn(move || {
            DesktopStage::start(move || DesktopStage::new(positions, colors, receiver, config));
        });

        Desktop {
            dim: PhantomData,
            layout: PhantomData,
            sender,
            brightness: 1.0,
        }
    }
}

/// Errors that can occur when using the Desktop driver.
#[derive(Debug)]
pub enum DesktopError {
    /// Sending to the render thread failed because it has already hung up.
    ChannelSend,
}

impl fmt::Display for DesktopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DesktopError::ChannelSend => write!(f, "render thread channel disconnected"),
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
    UpdateColors(Vec<Vec4>),
    /// Update the global brightness
    UpdateBrightness(f32),
    /// Terminate the rendering thread
    Quit,
}

// Implementation of the LedDriver trait for the Desktop driver
impl<Dim, Layout> LedDriver for Desktop<Dim, Layout>
where
    Layout: LayoutForDim<Dim>,
{
    type Error = DesktopError;
    type Color = Srgb;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        // Update brightness if it changed
        if self.brightness != brightness {
            self.brightness = brightness;
            self.sender.send(LedMessage::UpdateBrightness(brightness))?;
        }

        // Convert input colors to Vec4 for rendering
        let colors: Vec<Vec4> = pixels
            .into_iter()
            .map(|pixel| {
                let rgb: LinSrgb = Srgb::from_color(pixel).into_linear();
                Vec4::new(rgb.red, rgb.green, rgb.blue, 1.0)
            })
            .collect();

        // Send colors to the rendering thread
        self.sender.send(LedMessage::UpdateColors(colors))?;

        Ok(())
    }
}

impl<Dim, Layout> Drop for Desktop<Dim, Layout> {
    fn drop(&mut self) {
        let _ = self
            .sender
            .send(LedMessage::Quit)
            .expect("Failed to send quit message to render thread");
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
        // Rotate camera - left/right adjusts yaw, up/down adjusts pitch
        self.yaw -= delta_x * 0.01;
        self.pitch += delta_y * 0.01;

        // Clamp pitch to avoid gimbal lock
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
            // When looking straight up/down, use a different up vector to avoid gimbal lock
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

/// The rendering stage that handles the miniquad window and OpenGL drawing.
struct DesktopStage {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    positions: Vec<Vec3>,
    colors: Vec<Vec4>,
    brightness: f32,
    receiver: Receiver<LedMessage>,
    camera: Camera,
    config: DesktopConfig,

    // Mouse interaction state
    mouse_down: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
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
    pub fn new(
        positions: Vec<Vec3>,
        colors: Vec<Vec4>,
        receiver: Receiver<LedMessage>,
        config: DesktopConfig,
    ) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        // Use a bipyramid as the shape for each LED.
        let r = config.led_radius;
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            // positions          colors
            0.0,   -r, 0.0,       1.0, 0.0, 0.0, 1.0,
               r, 0.0, r,         0.0, 1.0, 0.0, 1.0,
               r, 0.0, -r,        0.0, 0.0, 1.0, 1.0,
              -r, 0.0, -r,        1.0, 1.0, 0.0, 1.0,
              -r, 0.0, r,         0.0, 1.0, 1.0, 1.0,
             0.0,   r, 0.0,       1.0, 0.0, 1.0, 1.0
        ];

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(vertices),
        );

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,    0, 2, 3,    0, 3, 4,    0, 4, 1,
            5, 1, 2,    5, 2, 3,    5, 3, 4,    5, 4, 1
        ];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(indices),
        );

        // Position and color buffer for instances
        let positions_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&positions),
        );

        let colors_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&colors),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer, positions_buffer, colors_buffer],
            index_buffer,
            images: vec![],
        };

        // Create shader for rendering LEDs
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        // Set up pipeline with instancing
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

        // Initialize camera and aspect ratio
        let (width, height) = window::screen_size();
        let camera = Camera::new(width / height, config.orthographic_view);

        Self {
            ctx,
            pipeline,
            bindings,
            positions,
            colors,
            brightness: 1.0,
            receiver,
            camera,
            config,
            mouse_down: false,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
        }
    }

    /// Process any pending messages from the main thread.
    fn process_messages(&mut self) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                LedMessage::UpdateColors(colors) => {
                    assert!(
                        colors.len() == self.colors.len(),
                        "Uh oh, number of pixels changed!"
                    );
                    self.colors = colors;
                }
                LedMessage::UpdateBrightness(brightness) => {
                    self.brightness = brightness;
                }
                LedMessage::Quit => {
                    window::quit();
                }
            }
        }
    }
}

impl EventHandler for DesktopStage {
    fn update(&mut self) {
        self.process_messages();
    }

    fn draw(&mut self) {
        // Apply brightness to all colors
        let bright_colors: Vec<Vec4> = self
            .colors
            .iter()
            .map(|c| {
                Vec4::new(
                    c.x * self.brightness,
                    c.y * self.brightness,
                    c.z * self.brightness,
                    c.w,
                )
            })
            .collect();

        // Update color buffer
        self.ctx.buffer_update(
            self.bindings.vertex_buffers[2],
            BufferSource::slice(&bright_colors),
        );

        // Get the view-projection matrix from the camera
        let view_proj = self.camera.view_projection_matrix();

        // Clear the frame with configured background color
        let (r, g, b, a) = self.config.background_color;
        self.ctx
            .begin_default_pass(PassAction::clear_color(r, g, b, a));

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms { mvp: view_proj }));

        self.ctx.draw(0, 24, self.positions.len() as i32);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }

    // Handle window resizing
    fn resize_event(&mut self, width: f32, height: f32) {
        self.camera.set_aspect_ratio(width / height);
    }

    // Handle mouse movement for camera rotation
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        if self.mouse_down {
            let dx = x - self.last_mouse_x;
            let dy = y - self.last_mouse_y;
            self.camera.rotate(dx, dy);
        }

        self.last_mouse_x = x;
        self.last_mouse_y = y;
    }

    // Handle mouse wheel for zoom
    fn mouse_wheel_event(&mut self, _x: f32, y: f32) {
        self.camera.zoom(y);
    }

    // Handle mouse button press
    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.mouse_down = true;
            self.last_mouse_x = x;
            self.last_mouse_y = y;
        }
    }

    // Handle mouse button release
    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.mouse_down = false;
        }
    }

    // Add keyboard controls
    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::R => {
                self.camera.reset();
            }
            KeyCode::O => {
                self.camera.toggle_projection_mode();
            }
            _ => {}
        }
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
