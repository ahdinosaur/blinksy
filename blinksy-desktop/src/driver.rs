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
//! ## Usage
//!
//! ```rust
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Shape2d, Vec2},
//!     patterns::{Rainbow, RainbowParams}
//! };
//! use blinksy_desktop::drivers::Desktop,
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
//!     let time = std::time::SystemTime::now()
//!         .duration_since(std::time::UNIX_EPOCH)
//!         .unwrap()
//!         .as_millis() as u64;
//!
//!     control.tick(time).unwrap();
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
            DesktopStage::start(positions, colors, receiver);
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
            DesktopStage::start(positions, colors, receiver);
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
        // Attempt to cleanly shut down the rendering thread
        let _ = self.sender.send(LedMessage::Quit);
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
}

impl DesktopStage {
    /// Start the rendering loop.
    pub fn start(positions: Vec<Vec3>, colors: Vec<Vec4>, receiver: Receiver<LedMessage>) {
        let conf = conf::Conf {
            window_title: "Blinksy".to_string(),
            window_width: 512,
            window_height: 512,
            high_dpi: true,
            ..Default::default()
        };

        miniquad::start(conf, move || {
            Box::new(Self::new(positions, colors, receiver))
        });
    }

    /// Create a new DesktopStage with the given LED positions and colors.
    pub fn new(positions: Vec<Vec3>, colors: Vec<Vec4>, receiver: Receiver<LedMessage>) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        // Define a simple LED shape (a bipyramid)
        let r = 0.05; // Radius of LED
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

        Self {
            ctx,
            pipeline,
            bindings,
            positions,
            colors,
            brightness: 1.0,
            receiver,
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

        // Set up camera/view
        let (width, height) = window::screen_size();
        let aspect = width / height;

        let proj = Mat4::orthographic_rh_gl(-aspect, aspect, -1.0, 1.0, -10.0, 10.0);
        let view = Mat4::IDENTITY;

        let view_proj = proj * view;

        self.ctx.begin_default_pass(Default::default());
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms { mvp: view_proj }));

        self.ctx.draw(0, 24, self.positions.len() as i32);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
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
