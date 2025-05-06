// text.rs
use fontdue::{Font, FontSettings};
use glam::{Mat4, Vec3, Vec4};
use miniquad::*;

pub struct TextLabel {
    texture: TextureId,
    width: u16,
    height: u16,
    position: Vec3,
    display_timer: f32,
    font: Font,
    pipeline: Pipeline,
    bindings: Bindings,
    visible: bool,
}

impl TextLabel {
    pub fn new(ctx: &mut dyn RenderingBackend) -> Self {
        // Load font (using Roboto Mono for good readability)
        let font_data = include_bytes!("../../resources/RobotoMono-Regular.ttf");
        let font = Font::from_bytes(font_data as &[u8], FontSettings::default())
            .expect("Failed to load font");

        // Create empty initial texture
        let pixels = vec![0u8; 4]; // 1x1 transparent pixel
        let texture = ctx.new_texture_from_rgba8(1, 1, &pixels);

        // Create quad vertices for billboard
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            // pos(x,y,z), uv(u,v)
            -0.5, -0.5, 0.0,   0.0, 1.0,
             0.5, -0.5, 0.0,   1.0, 1.0,
             0.5,  0.5, 0.0,   1.0, 0.0,
            -0.5,  0.5, 0.0,   0.0, 0.0,
        ];

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(vertices),
        );

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        // Create shader for text billboard
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: TEXT_VERTEX_SHADER,
                    fragment: TEXT_FRAGMENT_SHADER,
                },
                shader_meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::with_buffer("in_pos", VertexFormat::Float3, 0),
                VertexAttribute::with_buffer("in_uv", VertexFormat::Float2, 0),
            ],
            shader,
            PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                depth_test: Comparison::LessOrEqual,
                depth_write: true,
                ..Default::default()
            },
        );

        TextLabel {
            texture,
            width: 1,
            height: 1,
            position: Vec3::ZERO,
            display_timer: 0.0,
            font,
            pipeline,
            bindings,
            visible: false,
        }
    }

    pub fn set_text(&mut self, ctx: &mut dyn RenderingBackend, text: &str, position: Vec3) {
        self.position = position;
        self.display_timer = 3.0; // Show for 3 seconds
        self.visible = true;

        // Rasterize text with fontdue
        let mut layout =
            fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        layout.reset(&fontdue::layout::LayoutSettings {
            ..fontdue::layout::LayoutSettings::default()
        });

        let font_size = 24.0;
        layout.append(
            &[&self.font],
            &fontdue::layout::TextStyle::new(text, font_size, 0),
        );

        // Get layout bounds
        let glyphs = layout.glyphs();
        if glyphs.is_empty() {
            self.visible = false;
            return;
        }

        // Find bitmap dimensions
        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        for glyph in glyphs {
            let x_max = glyph.x + glyph.width as f32;
            let y_max = glyph.y + glyph.height as f32;
            max_x = max_x.max(x_max);
            max_y = max_y.max(y_max);
        }

        let width = max_x.ceil() as usize;
        let height = max_y.ceil() as usize;

        // Pad to ensure power of 2 dimensions (optimal for textures)
        let padded_width = width.next_power_of_two();
        let padded_height = height.next_power_of_two();

        // Create RGBA bitmap
        let mut pixels = vec![0u8; padded_width * padded_height * 4];

        // Render each glyph to the bitmap
        for (index, glyph) in glyphs.iter().enumerate() {
            let (_, bitmap) = self
                .font
                .rasterize_config(fontdue::layout::GlyphRasterConfig {
                    glyph_index: index as u16,
                    px: glyph.key.px,
                    font_hash: glyph.font_index,
                });

            let x_start = glyph.x as usize;
            let y_start = glyph.y as usize;

            for y in 0..glyph.height {
                for x in 0..glyph.width {
                    let src_idx = y * glyph.width + x;
                    let dst_idx = ((y_start + y) * padded_width + (x_start + x)) * 4;

                    if dst_idx + 3 < pixels.len() {
                        let coverage = bitmap[src_idx];
                        pixels[dst_idx] = 255; // R
                        pixels[dst_idx + 1] = 255; // G
                        pixels[dst_idx + 2] = 255; // B
                        pixels[dst_idx + 3] = coverage; // Alpha
                    }
                }
            }
        }

        // Update texture
        ctx.delete_texture(self.texture);
        self.texture =
            ctx.new_texture_from_rgba8(padded_width as u16, padded_height as u16, &pixels);
        ctx.texture_set_filter(self.texture, FilterMode::Linear, MipmapFilterMode::None);

        self.width = padded_width as u16;
        self.height = padded_height as u16;
        self.bindings.images[0] = self.texture;
    }

    pub fn update(&mut self, dt: f32) {
        if self.display_timer > 0.0 {
            self.display_timer -= dt;
            if self.display_timer <= 0.0 {
                self.visible = false;
            }
        }
    }

    pub fn draw(&self, ctx: &mut dyn RenderingBackend, view_proj: Mat4, camera_pos: Vec3) {
        if !self.visible {
            return;
        }

        // Calculate billboard matrix to always face camera
        let camera_dir = (camera_pos - self.position).normalize();
        let up = Vec3::new(0.0, 1.0, 0.0);
        let right = up.cross(camera_dir).normalize();
        let billboard_up = camera_dir.cross(right).normalize();

        // Scale based on text dimensions with a constant scale factor for readability
        let scale_factor = 0.01; // Adjust as needed
        let width_scale = self.width as f32 * scale_factor;
        let height_scale = self.height as f32 * scale_factor;

        let model = Mat4::from_cols(
            Vec4::new(
                right.x * width_scale,
                right.y * width_scale,
                right.z * width_scale,
                0.0,
            ),
            Vec4::new(
                billboard_up.x * height_scale,
                billboard_up.y * height_scale,
                billboard_up.z * height_scale,
                0.0,
            ),
            Vec4::new(camera_dir.x, camera_dir.y, camera_dir.z, 0.0),
            Vec4::new(self.position.x, self.position.y, self.position.z, 1.0),
        );

        let mvp = view_proj * model;

        // Draw billboard
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(UniformsSource::table(&TextUniforms { mvp }));
        ctx.draw(0, 6, 1);
    }
}

// Shader for text rendering
const TEXT_VERTEX_SHADER: &str = r#"#version 100
attribute vec3 in_pos;
attribute vec2 in_uv;

varying lowp vec2 texcoord;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(in_pos, 1.0);
    texcoord = in_uv;
}
"#;

const TEXT_FRAGMENT_SHADER: &str = r#"#version 100
precision mediump float;

varying lowp vec2 texcoord;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord);
}
"#;

#[repr(C)]
struct TextUniforms {
    mvp: Mat4,
}

fn shader_meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
        },
    }
}
