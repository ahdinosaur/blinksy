use crate::{
    color::{FromColor, Hsv, LinSrgb, Srgb},
    dimension::{Dim1d, Dim2d, LayoutForDim},
    driver::LedDriver,
    layout::{Layout1d, Layout2d},
};
use core::marker::PhantomData;
use glam::{vec3, Mat4, Vec3, Vec4};
use miniquad::*;
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct MiniquadError;

// Messages to communicate with the rendering thread
enum LedMessage {
    UpdateColors(Vec<Vec4>),
    UpdateBrightness(f32),
    Quit,
}

pub struct Graphics<Dim, Layout> {
    dim: PhantomData<Dim>,
    layout: PhantomData<Layout>,
    sender: Sender<LedMessage>,
    brightness: f32,
}

impl Graphics<Dim1d, ()> {
    pub fn new_1d<Layout>() -> Graphics<Dim1d, Layout>
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
            MiniquadStage::start(positions, colors, receiver);
        });

        Graphics {
            dim: PhantomData,
            layout: PhantomData,
            sender,
            brightness: 1.0,
        }
    }
}

impl Graphics<Dim2d, ()> {
    pub fn new_2d<Layout>() -> Graphics<Dim2d, Layout>
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
            MiniquadStage::start(positions, colors, receiver);
        });

        Graphics {
            dim: PhantomData,
            layout: PhantomData,
            sender,
            brightness: 1.0,
        }
    }
}

// Implement LedDriver trait for the miniquad driver
impl<Dim, Layout> LedDriver for Graphics<Dim, Layout>
where
    Layout: LayoutForDim<Dim>,
{
    type Error = MiniquadError;
    type Color = Srgb;

    fn write<I, C>(&mut self, pixels: I, brightness: f32) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = C>,
        Self::Color: FromColor<C>,
    {
        // Update brightness if it changed
        if self.brightness != brightness {
            self.brightness = brightness;
            self.sender
                .send(LedMessage::UpdateBrightness(brightness))
                .unwrap();
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
        self.sender.send(LedMessage::UpdateColors(colors)).unwrap();

        Ok(())
    }
}

impl<Dim, Layout> Drop for Graphics<Dim, Layout> {
    fn drop(&mut self) {
        let _ = self.sender.send(LedMessage::Quit);
    }
}

// The miniquad rendering stage - handles all rendering logic
struct MiniquadStage {
    ctx: Box<dyn RenderingBackend>,
    pipeline: Pipeline,
    bindings: Bindings,
    positions: Vec<Vec3>,
    colors: Vec<Vec4>,
    brightness: f32,
    receiver: Receiver<LedMessage>,
}

impl MiniquadStage {
    pub fn start(positions: Vec<Vec3>, colors: Vec<Vec4>, receiver: Receiver<LedMessage>) {
        let conf = conf::Conf {
            window_title: "Blinksy LED Simulator".to_string(),
            window_width: 800,
            window_height: 600,
            high_dpi: true,
            ..Default::default()
        };

        miniquad::start(conf, move || {
            Box::new(Self::new(positions, colors, receiver))
        });
    }

    pub fn new(positions: Vec<Vec3>, colors: Vec<Vec4>, receiver: Receiver<LedMessage>) -> Self {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let r = 0.05;
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

impl EventHandler for MiniquadStage {
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

mod shader {
    use miniquad::*;

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

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;

    void main() {
        gl_FragColor = color;
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}
