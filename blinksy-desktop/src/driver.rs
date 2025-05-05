//! # Desktop Simulation Driver
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
//! ```rust,no_run
//! use blinksy::{
//!     ControlBuilder,
//!     layout2d,
//!     layout::{Shape2d, Vec2},
//!     patterns::{Rainbow, RainbowParams}
//! };
//! use blinksy_desktop::{drivers::Desktop, time::elapsed_in_ms};
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
//!     control.tick(elapsed_in_ms()).unwrap();
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
use std::sync::{
    mpsc::{channel, Receiver, SendError, Sender},
    Arc,
};
use std::time::{Duration, Instant};
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent, WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

/// Configuration options for the desktop simulator.
///
/// Allows customizing the appearance and behavior of the LED simulator window.
#[derive(Clone, Debug)]
pub struct DesktopConfig {
    /// Window title
    pub window_title: String,

    /// Window width in pixels
    pub window_width: u32,

    /// Window height in pixels
    pub window_height: u32,

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
/// representation of your LED layout using wgpu.
///
/// # Type Parameters
///
/// * `Dim` - The dimension marker (Dim1d or Dim2d)
/// * `Layout` - The specific layout type
pub struct Desktop<Dim, Layout> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    brightness: f32,
    sender: Sender<LedMessage>,
    is_window_closed: Arc<std::sync::atomic::AtomicBool>,
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
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);
        for x in Layout::points() {
            positions.push(vec3(x, 0.0, 0.0));
        }

        let colors = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); Layout::PIXEL_COUNT];
        let (sender, receiver) = channel();
        let is_window_closed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_window_closed_clone = is_window_closed.clone();

        std::thread::spawn(move || {
            let renderer =
                WgpuRenderer::new(positions, colors, receiver, config, is_window_closed_clone);
            renderer.run();
        });

        Desktop {
            dim: PhantomData,
            layout: PhantomData,
            brightness: 1.0,
            sender,
            is_window_closed,
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
        let mut positions = Vec::with_capacity(Layout::PIXEL_COUNT);
        for point in Layout::points() {
            positions.push(vec3(point.x, point.y, 0.0));
        }

        let colors = vec![Vec4::new(0.0, 0.0, 0.0, 1.0); Layout::PIXEL_COUNT];
        let (sender, receiver) = channel();
        let is_window_closed = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let is_window_closed_clone = is_window_closed.clone();

        std::thread::spawn(move || {
            let renderer =
                WgpuRenderer::new(positions, colors, receiver, config, is_window_closed_clone);
            renderer.run();
        });

        Desktop {
            dim: PhantomData,
            layout: PhantomData,
            brightness: 1.0,
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
    UpdateColors(Vec<Vec4>),

    /// Update the global brightness
    UpdateBrightness(f32),

    /// Terminate the rendering thread
    Quit,
}

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
        if self.brightness != brightness {
            self.brightness = brightness;
            self.send(LedMessage::UpdateBrightness(brightness))?;
        }

        let colors: Vec<Vec4> = pixels
            .into_iter()
            .map(|pixel| {
                let rgb: LinSrgb = Srgb::from_color(pixel).into_linear();
                Vec4::new(rgb.red, rgb.green, rgb.blue, 1.0)
            })
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
            Mat4::orthographic_rh(
                -vertical_size * self.aspect_ratio,
                vertical_size * self.aspect_ratio,
                -vertical_size,
                vertical_size,
                -100.0,
                100.0,
            )
        } else {
            Mat4::perspective_rh(self.fov, self.aspect_ratio, 0.1, 100.0)
        }
    }

    /// Get the combined view-projection matrix
    fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

// Vertex shader for LED rendering - WGSL version
const VERTEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) instance_position: vec3<f32>,
    @location(3) instance_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Uniforms {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.mvp * vec4<f32>(input.position + input.instance_position, 1.0);
    out.color = input.instance_color;
    return out;
}
"#;

// Fragment shader for LED rendering - WGSL version
const FRAGMENT_SHADER: &str = r#"
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};
"#;

// Data structures for wgpu implementation
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {
    position: [f32; 3],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}

struct WgpuRenderer<'window> {
    event_loop: EventLoop<()>,
    window: Window,
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    positions: Vec<Vec3>,
    colors: Vec<Vec4>,
    brightness: f32,
    receiver: Receiver<LedMessage>,
    camera: Camera,
    desktop_config: DesktopConfig,
    is_window_closed: Arc<std::sync::atomic::AtomicBool>,
    mouse_down: bool,
    last_mouse_pos: PhysicalPosition<f64>,
    indices: Vec<u16>,
}

impl<'window> WgpuRenderer<'window> {
    async fn init_wgpu(
        window: &'window Window,
    ) -> (
        wgpu::Surface<'window>,
        wgpu::Device,
        wgpu::Queue,
        wgpu::SurfaceConfiguration,
    ) {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            desired_maximum_frame_latency: 2,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        (surface, device, queue, config)
    }

    fn new(
        positions: Vec<Vec3>,
        colors: Vec<Vec4>,
        receiver: Receiver<LedMessage>,
        config: DesktopConfig,
        is_window_closed: Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title(&config.window_title)
            .with_inner_size(winit::dpi::PhysicalSize::new(
                config.window_width,
                config.window_height,
            ))
            .build(&event_loop)
            .unwrap();

        // Set up camera
        let aspect_ratio = config.window_width as f32 / config.window_height as f32;
        let camera = Camera::new(aspect_ratio, config.orthographic_view);

        // Setup wgpu asynchronously, but block until complete
        let (surface, device, queue, surface_config) = pollster::block_on(Self::init_wgpu(&window));

        // Create shader module from WGSL
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(VERTEX_SHADER.into()),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: wgpu::ShaderSource::Wgsl(FRAGMENT_SHADER.into()),
        });

        // Create vertex buffer for LED model
        let r = config.led_radius;
        #[rustfmt::skip]
        let vertices = [
            Vertex { position: [0.0, -r, 0.0], color: [1.0, 0.0, 0.0, 1.0] },
            Vertex { position: [r, 0.0, r], color: [0.0, 1.0, 0.0, 1.0] },
            Vertex { position: [r, 0.0, -r], color: [0.0, 0.0, 1.0, 1.0] },
            Vertex { position: [-r, 0.0, -r], color: [1.0, 1.0, 0.0, 1.0] },
            Vertex { position: [-r, 0.0, r], color: [0.0, 1.0, 1.0, 1.0] },
            Vertex { position: [0.0, r, 0.0], color: [1.0, 0.0, 1.0, 1.0] },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer
        #[rustfmt::skip]
        let indices: Vec<u16> = vec![
            0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 1,
            5, 1, 2, 5, 2, 3, 5, 3, 4, 5, 4, 1
        ];

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create instance buffer with positions and colors
        let mut instances = Vec::with_capacity(positions.len());
        for (pos, color) in positions.iter().zip(colors.iter()) {
            instances.push(Instance {
                position: [pos.x, pos.y, pos.z],
                color: [color.x, color.y, color.z, color.w],
            });
        }

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // Create uniform buffer for MVP matrix
        let mvp = camera.view_projection_matrix();
        let uniform_data = Uniforms {
            mvp: mvp.to_cols_array_2d(),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout for uniforms
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("bind_group_layout"),
        });

        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        WgpuRenderer {
            event_loop,
            window,
            surface,
            device,
            queue,
            config: surface_config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            uniform_buffer,
            uniform_bind_group,
            positions,
            colors,
            brightness: 1.0,
            receiver,
            camera,
            desktop_config: config,
            is_window_closed,
            mouse_down: false,
            last_mouse_pos: PhysicalPosition::new(0.0, 0.0),
            indices,
        }
    }

    fn process_messages(&mut self) {
        while let Ok(message) = self.receiver.try_recv() {
            match message {
                LedMessage::UpdateColors(colors) => {
                    assert!(
                        colors.len() == self.colors.len(),
                        "Uh oh, number of pixels changed!"
                    );
                    self.colors = colors;
                    self.update_instance_buffer();
                }
                LedMessage::UpdateBrightness(brightness) => {
                    self.brightness = brightness;
                    self.update_instance_buffer();
                }
                LedMessage::Quit => {
                    self.is_window_closed
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                }
            }
        }
    }

    fn update_instance_buffer(&mut self) {
        let mut instances = Vec::with_capacity(self.positions.len());

        for (pos, color) in self.positions.iter().zip(self.colors.iter()) {
            instances.push(Instance {
                position: [pos.x, pos.y, pos.z],
                color: [
                    color.x * self.brightness,
                    color.y * self.brightness,
                    color.z * self.brightness,
                    color.w,
                ],
            });
        }

        self.queue
            .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    fn update_uniform_buffer(&mut self) {
        let mvp = self.camera.view_projection_matrix();
        let uniform_data = Uniforms {
            mvp: mvp.to_cols_array_2d(),
        };

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[uniform_data]),
        );
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera
                .set_aspect_ratio(new_size.width as f32 / new_size.height as f32);
            self.update_uniform_buffer();
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create depth texture
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let (r, g, b, a) = self.desktop_config.background_color;

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: r as f64,
                            g: g as f64,
                            b: b as f64,
                            a: a as f64,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(
                0..self.indices.len() as u32,
                0,
                0..self.positions.len() as u32,
            );
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn run(mut self) {
        let mut last_render_time = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested => {
                        self.is_window_closed
                            .store(true, std::sync::atomic::Ordering::Relaxed);
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.resize(**new_inner_size);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => match keycode {
                        VirtualKeyCode::R => {
                            self.camera.reset();
                            self.update_uniform_buffer();
                        }
                        VirtualKeyCode::O => {
                            self.camera.toggle_projection_mode();
                            self.update_uniform_buffer();
                        }
                        VirtualKeyCode::Escape => {
                            self.is_window_closed
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    } => {
                        self.mouse_down = *state == ElementState::Pressed;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        if self.mouse_down {
                            let dx = position.x - self.last_mouse_pos.x;
                            let dy = position.y - self.last_mouse_pos.y;
                            self.camera.rotate(dx as f32, dy as f32);
                            self.update_uniform_buffer();
                        }
                        self.last_mouse_pos = *position;
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let scroll_amount = match delta {
                            MouseScrollDelta::LineDelta(_, y) => *y,
                            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                                *y as f32 / 100.0
                            }
                        };
                        self.camera.zoom(scroll_amount);
                        self.update_uniform_buffer();
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    // Process any incoming LED messages
                    self.process_messages();

                    // Throttle rendering to ~60 FPS
                    let now = Instant::now();
                    let duration = now.duration_since(last_render_time);
                    if duration >= Duration::from_millis(16) {
                        self.window.request_redraw();
                        last_render_time = now;
                    }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    match self.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => self.resize(self.window.inner_size()),
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        Err(e) => eprintln!("Render error: {:?}", e),
                    }
                }
                _ => {}
            }
        });
    }
}
