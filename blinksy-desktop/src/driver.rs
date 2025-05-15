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
    color::{ColorCorrection, FromColor, LinearSrgb, Srgb},
    dimension::{Dim1d, Dim2d, LayoutForDim},
    driver::LedDriver,
    layout::{Layout1d, Layout2d},
};
use core::{fmt, marker::PhantomData};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{channel, Receiver, SendError, Sender},
    Arc, Mutex,
};
use three_d::{
    degrees, egui, radians, Camera, CameraControl, ColorFormat, ColorMaterial, Context, CpuMesh,
    Depth, DepthFormat, FrameOutput, Gm, Indices, InstancedMesh, Mat4, Mesh, NormalsMaterial,
    Object, OrbitControl, PhysicalMaterial, Position, Positions, Positions as ThreePositions,
    RenderTarget, Srgba, Vec3, Viewport, Window, WindowSettings,
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
/// representation of your LED layout using three-d.
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
    is_window_closed: Arc<AtomicBool>,
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
            positions.push(Vec3::new(x, 0.0, 0.0));
        }

        let (sender, receiver) = channel();
        let is_window_closed = Arc::new(AtomicBool::new(false));
        let is_window_closed_2 = is_window_closed.clone();

        std::thread::spawn(move || {
            SimulationWindow::start(positions, receiver, config, is_window_closed_2);
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
            positions.push(Vec3::new(point.x, point.y, 0.0));
        }

        let (sender, receiver) = channel();
        let is_window_closed = Arc::new(AtomicBool::new(false));
        let is_window_closed_2 = is_window_closed.clone();

        std::thread::spawn(move || {
            SimulationWindow::start(positions, receiver, config, is_window_closed_2);
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
        if self.is_window_closed.load(Ordering::Relaxed) {
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

impl<Dim, Layout> LedDriver for Desktop<Dim, Layout>
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

/// Simulation window that handles the three-d rendering and GUI
struct SimulationWindow {
    window: Window,
    camera: Camera,
    orbit_control: OrbitControl,
    led_mesh: Mesh,
    led_positions: Vec<Vec3>,
    led_materials: Vec<ColorMaterial>,
    colors: Vec<LinearSrgb>,
    brightness: f32,
    correction: ColorCorrection,
    receiver: Receiver<LedMessage>,
    selected_led: Option<usize>,
    is_window_closed: Arc<AtomicBool>,
    config: DesktopConfig,
    use_orthographic: bool,
}

impl SimulationWindow {
    /// Start the simulation window
    fn start(
        positions: Vec<Vec3>,
        receiver: Receiver<LedMessage>,
        config: DesktopConfig,
        is_window_closed: Arc<AtomicBool>,
    ) {
        // Create window
        let window = Window::new(WindowSettings {
            title: config.window_title.clone(),
            max_size: Some((config.window_width, config.window_height)),
            ..Default::default()
        })
        .unwrap();

        // Get context
        let context = window.gl();

        // Create camera
        let mut camera = Camera::new_perspective(
            window.viewport(),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            100.0,
        );

        // If orthographic is the default, switch to it
        if config.orthographic_view {
            camera = Camera::new_orthographic(
                window.viewport(),
                Vec3::new(0.0, 0.0, 4.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                2.0,
                0.1,
                100.0,
            );
        }

        // Create controls for the camera
        let mut orbit_control = OrbitControl::new(*camera.target(), 1.0, 10.0);

        // Create LED mesh
        let led_mesh = create_led_mesh(&context, config.led_radius);

        // Create initial materials for all LEDs (default color)
        let led_materials = (0..positions.len())
            .map(|_| ColorMaterial {
                color: Srgba::new(0.0, 0.0, 0.0, 1.0),
                ..Default::default()
            })
            .collect();

        // Create simulation window
        let mut simulation = SimulationWindow {
            window,
            camera,
            orbit_control,
            led_mesh,
            led_positions: positions,
            led_materials,
            colors: Vec::new(),
            brightness: 1.0,
            correction: ColorCorrection::default(),
            receiver,
            selected_led: None,
            is_window_closed,
            config,
            use_orthographic: config.orthographic_view,
        };

        // Run the window event loop
        simulation.run();
    }

    /// Process any pending messages from the main thread
    fn process_messages(&mut self) -> bool {
        let mut should_quit = false;

        while let Ok(message) = self.receiver.try_recv() {
            match message {
                LedMessage::UpdateColors(colors) => {
                    self.colors = colors;
                    self.update_led_materials();
                }
                LedMessage::UpdateBrightness(brightness) => {
                    self.brightness = brightness;
                    self.update_led_materials();
                }
                LedMessage::UpdateColorCorrection(correction) => {
                    self.correction = correction;
                    self.update_led_materials();
                }
                LedMessage::Quit => {
                    should_quit = true;
                }
            }
        }

        should_quit
    }

    /// Update LED materials based on current colors, brightness, and correction
    fn update_led_materials(&mut self) {
        for (i, color) in self.colors.iter().enumerate() {
            if i < self.led_materials.len() {
                // Apply brightness
                let mut red = color.red * self.brightness;
                let mut green = color.green * self.brightness;
                let mut blue = color.blue * self.brightness;

                // Apply color correction
                red *= self.correction.red;
                green *= self.correction.green;
                blue *= self.correction.blue;

                // Convert to sRGB
                let Srgb { red, green, blue } = LinearSrgb::new(red, green, blue).to_srgb();

                // Update material
                self.led_materials[i].color = Srgba::new(red, green, blue, 1.0);
            }
        }
    }

    /// Try to select an LED at the given screen position
    fn try_select_led(&mut self, position: (f32, f32)) {
        // Convert screen position to a ray
        let ray = self.camera.view_ray_from_pixel(position);

        // Find the closest LED that intersects with the ray
        let mut closest_led = None;
        let mut closest_distance = f32::MAX;

        for (i, &pos) in self.led_positions.iter().enumerate() {
            // Simple sphere-ray intersection
            let sphere_center = pos;
            let sphere_radius = self.config.led_radius;

            let oc = ray.origin - sphere_center;
            let a = ray.direction.dot(ray.direction);
            let b = 2.0 * oc.dot(ray.direction);
            let c = oc.dot(oc) - sphere_radius * sphere_radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant > 0.0 {
                let t = (-b - discriminant.sqrt()) / (2.0 * a);
                if t > 0.0 && t < closest_distance {
                    closest_distance = t;
                    closest_led = Some(i);
                }
            }
        }

        self.selected_led = closest_led;
    }

    /// Toggle between orthographic and perspective projection
    fn toggle_projection_mode(&mut self) {
        self.use_orthographic = !self.use_orthographic;

        let viewport = self.window.viewport();
        let target = *self.camera.target();
        let position = *self.camera.position();
        let up = *self.camera.up();

        if self.use_orthographic {
            self.camera = Camera::new_orthographic(viewport, position, target, up, 2.0, 0.1, 100.0);
        } else {
            self.camera =
                Camera::new_perspective(viewport, position, target, up, degrees(45.0), 0.1, 100.0);
        }
    }

    /// Reset camera to default position
    fn reset_camera(&mut self) {
        let viewport = self.window.viewport();

        if self.use_orthographic {
            self.camera = Camera::new_orthographic(
                viewport,
                Vec3::new(0.0, 0.0, 4.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                2.0,
                0.1,
                100.0,
            );
        } else {
            self.camera = Camera::new_perspective(
                viewport,
                Vec3::new(0.0, 0.0, 4.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                degrees(45.0),
                0.1,
                100.0,
            );
        }

        self.orbit_control = OrbitControl::new(*self.camera.target(), 1.0, 10.0);
    }

    /// Create and show the UI for selected LED information
    fn render_led_info(&self, gui_context: &egui::Context) {
        if let Some(led_idx) = self.selected_led {
            if led_idx < self.colors.len() {
                let pos = self.led_positions[led_idx];
                let color = self.colors[led_idx];

                let (red, green, blue) = (color.red, color.green, color.blue);

                // Apply brightness
                let (bright_red, bright_green, bright_blue) = (
                    red * self.brightness,
                    green * self.brightness,
                    blue * self.brightness,
                );

                // Apply color correction
                let (correct_red, correct_green, correct_blue) = (
                    bright_red * self.correction.red,
                    bright_green * self.correction.green,
                    bright_blue * self.correction.blue,
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
                    .show(gui_context, |ui| {
                        ui.label(format!("LED Index: {}", led_idx));
                        ui.label(format!(
                            "Position: ({:.3}, {:.3}, {:.3})",
                            pos.x, pos.y, pos.z
                        ));

                        // Display raw RGB values
                        ui.label(format!(
                            "Linear RGB: R={:.3}, G={:.3}, B={:.3}",
                            red, green, blue
                        ));

                        // Display global brightness
                        ui.label(format!("Global Brightness: {:.3}", self.brightness));

                        // Display brightness-adjusted RGB values
                        ui.label(format!(
                            "Brightness-adjusted RGB: R={:.3}, G={:.3}, B={:.3}",
                            bright_red, bright_green, bright_blue
                        ));

                        // Display global color correction
                        ui.label(format!(
                            "Global Color Correction: R={:.3}, G={:.3}, B={:.3}",
                            self.correction.red, self.correction.green, self.correction.blue
                        ));

                        // Display correction-adjusted RGB values
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
                        let color_preview = egui::Color32::from_rgb(
                            (srgb_red * 255.0) as u8,
                            (srgb_green * 255.0) as u8,
                            (srgb_blue * 255.0) as u8,
                        );

                        let rect = ui.available_rect_before_wrap();
                        let color_rect =
                            egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), 30.0));
                        ui.painter().rect_filled(color_rect, 4.0, color_preview);
                        ui.add_space(40.0); // Space after the color preview

                        // Deselect button
                        if ui.button("Deselect").clicked() {
                            // We can't modify self.selected_led here due to borrowing,
                            // but we can set it to None in the event loop
                        }
                    });
            }
        }
    }

    /// Run the main event loop
    fn run(&mut self) {
        let context = self.window.gl();
        let (r, g, b, a) = self.config.background_color;

        // Create GUI context
        let mut gui = three_d::GUI::new(&context);
        let mut should_deselect = false;

        // Main event loop
        self.window.render_loop(move |mut frame_input| {
            // Process any pending messages
            let should_quit = self.process_messages();
            if should_quit {
                return FrameOutput::NotRedraw;
            }

            // Check for window close
            if frame_input.events.iter().any(|event| {
                matches!(
                    event,
                    three_d::Event::WindowCloseRequested(_)
                        | three_d::Event::KeyRelease {
                            key: three_d::Key::Escape,
                            ..
                        }
                )
            }) {
                self.is_window_closed.store(true, Ordering::Relaxed);
                return FrameOutput::Quit;
            }

            // Handle key events
            for event in &frame_input.events {
                match event {
                    three_d::Event::KeyPress { key, .. } => match key {
                        three_d::Key::R => self.reset_camera(),
                        three_d::Key::O => self.toggle_projection_mode(),
                        _ => {}
                    },
                    three_d::Event::MousePress {
                        position, button, ..
                    } => {
                        if *button == three_d::MouseButton::Left {
                            self.try_select_led(*position);
                        }
                    }
                    _ => {}
                }
            }

            // Handle orbit controls (camera movement)
            self.orbit_control
                .handle_events(&mut self.camera, &mut frame_input.events);

            // Update viewport if window was resized
            self.camera.set_viewport(frame_input.viewport);

            // If should_deselect was set in UI, clear selection
            if should_deselect {
                self.selected_led = None;
                should_deselect = false;
            }

            // Generate LED objects
            let led_objects: Vec<_> = self
                .led_positions
                .iter()
                .enumerate()
                .map(|(i, &position)| {
                    let highlighted = Some(i) == self.selected_led;
                    let material = if highlighted {
                        // Create a highlighted version of the material for selected LED
                        let mut highlight_material = self.led_materials[i].clone();
                        highlight_material.color = Srgba::new(1.0, 1.0, 1.0, 1.0);
                        highlight_material
                    } else {
                        self.led_materials[i].clone()
                    };

                    Gm::new(self.led_mesh.clone(), material)
                        .set_transformation(Mat4::from_translation(position))
                })
                .collect();

            // Get screen and render target
            frame_input.screen().clear(Srgba::new(r, g, b, a)).render(
                &self.camera,
                led_objects.iter(),
                &[],
            );

            // Process GUI
            let mut redraw = frame_input.first_frame;
            gui.update(
                &mut frame_input.events,
                frame_input.accumulated_time,
                frame_input.viewport,
                frame_input.device_pixel_ratio,
                |gui_context| {
                    self.render_led_info(gui_context);

                    // Check for deselect button click
                    if self.selected_led.is_some() {
                        gui_context.ctx_mut().memory_mut(|mem| {
                            if mem
                                .data
                                .get_temp::<bool>("deselect_clicked")
                                .unwrap_or(false)
                            {
                                should_deselect = true;
                                mem.data.insert_temp("deselect_clicked", false);
                            }
                        });
                    }

                    redraw = gui_context.ctx_mut().wants_repaint();
                },
            );

            if redraw {
                FrameOutput::Redraw
            } else {
                FrameOutput::Wait
            }
        });
    }
}
