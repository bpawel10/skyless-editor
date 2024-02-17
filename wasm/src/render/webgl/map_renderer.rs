use super::super::Render;
use crate::project::Project;
use itertools::Itertools;
use js_sys::Float32Array;
use model::{attributes::Item, Attribute, Position};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{console, HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};
use webgl_matrix::Mat3;

static VERTEX_SHADER: &str = include_str!("shaders/vertex.glsl");
static FRAGMENT_SHADER: &str = include_str!("shaders/fragment.glsl");

#[wasm_bindgen]
#[derive(Clone)]
pub struct WebGLMapRenderer {
    #[wasm_bindgen(skip)]
    pub program: WebGlProgram,
    #[wasm_bindgen(skip)]
    pub gl: WebGl2RenderingContext,
    vertices_count: usize,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebGLSetupError {
    ShaderCompilation,
    ShaderUnknown,
    ProgramCreation,
    ProgramUnknown,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum WebGLRenderError {
    Something, // FIXME:
}

// FIXME: most of these consts should be dynamically calculated based on webgl limit on that specific machine
const ATLAS_WIDTH: u16 = 16384;
const ATLAS_HEIGHT: u16 = 16384;

const TILE_SIZE: u16 = 32;
const TEXTURE_SIZE: u16 = 128;
const TEXTURE_PADDING: u16 = 32;
const TEXTURES_IN_ROW: u16 = 128;

const FLOOR_LEVEL: u8 = 7;

#[wasm_bindgen]
impl WebGLMapRenderer {
    fn canvas(&self) -> HtmlCanvasElement {
        self.gl.canvas().unwrap().dyn_into().unwrap()
    }

    fn compile_shader(
        gl: &WebGl2RenderingContext,
        r#type: u32,
        source: &str,
    ) -> Result<WebGlShader, WebGLSetupError> {
        let shader = gl
            .create_shader(r#type)
            .ok_or(WebGLSetupError::ShaderCompilation)?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            console::log_1(&JsValue::from(gl.get_shader_info_log(&shader)));
            // TODO: pass shader info log to this error
            Err(WebGLSetupError::ShaderUnknown)
        }
    }

    fn link_program(
        gl: &WebGl2RenderingContext,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, WebGLSetupError> {
        let program = gl
            .create_program()
            .ok_or(WebGLSetupError::ProgramCreation)?;

        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            // TODO: pass program info log to this error
            Err(WebGLSetupError::ProgramUnknown)
        }
    }

    fn fill_buffers(&mut self, project: &Project) -> Result<(), WebGLSetupError> {
        let mut positions = Vec::new();
        let mut texcoords = Vec::new();

        let current_level = 0;

        let tiles = project
            .data
            .world
            .tiles()
            .iter()
            .filter(|(pos, _)| {
                if current_level <= FLOOR_LEVEL {
                    pos.z >= current_level && pos.z <= FLOOR_LEVEL
                } else {
                    pos.z > FLOOR_LEVEL && pos.z <= current_level
                }
            })
            .sorted_by(
                |(a, _), (b, _)| {
                    if a.z == b.z {
                        a.cmp(b)
                    } else {
                        b.z.cmp(&a.z)
                    }
                },
            );

        // TODO: instanced drawing
        for (Position { x, y, z, .. }, tile) in tiles
        // TODO: try BTreeMap that is already sorted
        {
            for entity in tile.entities.iter() {
                if let Some(Attribute::Item(Item(item_id))) = entity.attributes.get("item") {
                    if let Some(item) = project.data.assets.items.get(item_id) {
                        let texture = item.texture();

                        let offset = z - current_level;
                        let tile_width = texture.width / TILE_SIZE;
                        let tile_height = texture.height / TILE_SIZE;
                        let x_b = (x + 1 + offset as u16) as f32;
                        let y_b = (y + 1 + offset as u16) as f32;
                        let x_a = x_b - tile_width as f32;
                        let y_a = y_b - tile_height as f32;

                        positions.append(&mut vec![
                            x_a, y_a, x_a, y_b, x_b, y_a, x_b, y_a, x_a, y_b, x_b, y_b,
                        ]);

                        let tx = (item.id % TEXTURES_IN_ROW) as u16;
                        let ty = (item.id as f32 / TEXTURES_IN_ROW as f32).floor() as u16;

                        let tx_a = (tx * TEXTURE_SIZE + TEXTURE_PADDING) as f32;
                        let ty_a = (ty * TEXTURE_SIZE + TEXTURE_PADDING) as f32;
                        let tx_b = tx_a + texture.width as f32;
                        let ty_b = ty_a + texture.height as f32;

                        texcoords.append(&mut vec![
                            tx_a, ty_a, tx_a, ty_b, tx_b, ty_a, tx_b, ty_a, tx_a, ty_b, tx_b, ty_b,
                        ]);
                    }
                }
            }
        }

        self.vertices_count = positions.len() / 2;

        let position_location = self.gl.get_attrib_location(&self.program, "a_position");
        let position_buffer = self.gl.create_buffer();
        self.gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            position_buffer.as_ref(),
        );
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &Float32Array::view(&positions),
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        self.gl.vertex_attrib_pointer_with_i32(
            position_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(position_location as u32);

        let texcoord_location = self.gl.get_attrib_location(&self.program, "a_texcoord");
        let texcoord_buffer = self.gl.create_buffer();
        self.gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            texcoord_buffer.as_ref(),
        );
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &Float32Array::view(&texcoords),
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        self.gl.vertex_attrib_pointer_with_i32(
            texcoord_location as u32,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(texcoord_location as u32);

        Ok(())
    }

    fn fill_textures(&self, project: &Project) -> Result<(), WebGLSetupError> {
        let texture = self.gl.create_texture();
        self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
        self.gl.tex_storage_2d(
            WebGl2RenderingContext::TEXTURE_2D,
            5, // how many mipmaps, it's 5 because padding is 32 which is 2^5
            WebGl2RenderingContext::RGBA8,
            ATLAS_WIDTH as i32,
            ATLAS_HEIGHT as i32,
        );

        for item in project.data.assets.items.values() {
            let texture = item.texture();

            let tx = ((item.id % TEXTURES_IN_ROW) * TEXTURE_SIZE) as i32;
            let ty =
                ((item.id as f32 / TEXTURES_IN_ROW as f32).floor() as u16 * TEXTURE_SIZE) as i32;

            self.gl
                .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    tx,
                    ty,
                    (texture.width + 2 * TEXTURE_PADDING) as i32,
                    (texture.height + 2 * TEXTURE_PADDING) as i32,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    Some(&texture.rgba_bytes(TEXTURE_PADDING as usize)),
                )
                .unwrap(); // FIXME: unwrap
        }

        let texture_location = self.gl.get_uniform_location(&self.program, "u_texture");
        self.gl.uniform1i(texture_location.as_ref(), 0);
        self.gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        self.gl
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        self.gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
        );
        self.gl.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32,
        );

        Ok(())
    }

    fn configure_webgl(&self) -> Result<(), WebGLSetupError> {
        let tile_size_location = self.gl.get_uniform_location(&self.program, "u_tile_size");
        self.gl
            .uniform1f(tile_size_location.as_ref(), TILE_SIZE as f32);

        let resolution_location = self.gl.get_uniform_location(&self.program, "u_resolution");
        self.gl.uniform2f(
            resolution_location.as_ref(),
            self.canvas().width() as f32,
            self.canvas().height() as f32,
        );

        let texture_atlas_size_location = self
            .gl
            .get_uniform_location(&self.program, "u_texture_atlas_size");
        self.gl.uniform2f(
            texture_atlas_size_location.as_ref(),
            ATLAS_WIDTH as f32,
            ATLAS_HEIGHT as f32,
        );

        self.gl.enable(WebGl2RenderingContext::BLEND);
        self.gl.blend_func(
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        self.gl.viewport(
            0,
            0,
            self.canvas().width() as i32,
            self.canvas().height() as i32,
        );

        Ok(())
    }

    // workaround because it's not possible to use wasm_bindgen macro directly on trait implementation methods
    #[wasm_bindgen(constructor)]
    pub fn new_wasm(
        project: &Project,
        canvas: HtmlCanvasElement,
    ) -> Result<WebGLMapRenderer, JsValue> {
        WebGLMapRenderer::new(project, canvas).map_err(|error| error.into())
    }

    // workaround because it's not possible to use wasm_bindgen macro directly on trait implementation methods
    #[wasm_bindgen(js_name = render)]
    pub fn render_wasm(&self, transformation: &Float32Array) -> Result<(), JsValue> {
        let mat3 = Mat3::try_from(transformation.to_vec()).unwrap();
        self.render(&mat3).map_err(|error| error.into())
    }
}

impl Render<WebGLSetupError, WebGLRenderError> for WebGLMapRenderer {
    fn new(project: &Project, canvas: HtmlCanvasElement) -> Result<Self, WebGLSetupError> {
        let opts = js_sys::Object::new();
        js_sys::Reflect::set(
            &opts,
            &"powerPreference".into(),
            &String::from("high-performance").into(),
        )
        .unwrap();

        // FIXME: all unwraps
        let gl = canvas
            .get_context_with_context_options("webgl2", &opts)
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();

        let vertex_shader =
            Self::compile_shader(&gl, WebGl2RenderingContext::VERTEX_SHADER, VERTEX_SHADER)?;
        let fragment_shader = Self::compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER,
        )?;

        let program = Self::link_program(&gl, &vertex_shader, &fragment_shader)?;
        gl.use_program(Some(&program));

        let mut renderer = WebGLMapRenderer {
            program,
            gl,
            vertices_count: 0,
        };

        renderer.fill_buffers(project)?;
        renderer.fill_textures(project)?;
        renderer.configure_webgl()?;

        Ok(renderer)
    }

    fn render(&self, transformation: &Mat3) -> Result<(), WebGLRenderError> {
        let transformation_location = self
            .gl
            .get_uniform_location(&self.program, "u_transformation");
        self.gl.uniform_matrix3fv_with_f32_array(
            transformation_location.as_ref(),
            false,
            transformation,
        );

        self.gl.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
        self.gl.clear_color(0., 0., 0., 1.);

        self.gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices_count as i32,
        );

        Ok(())
    }
}
