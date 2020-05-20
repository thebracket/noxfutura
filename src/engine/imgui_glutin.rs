extern crate imgui;

use imgui::{Context,Ui};
use std::mem;
use super::support::gl::*;
use super::support::gl::types::*;
use super::GL;

pub struct Renderer {
  program: GLuint,
  locs: Locs,
  vbo: GLuint,
  ebo: GLuint,
  font_texture: GLuint,
}

struct Locs {
  texture: GLint,
  proj_mtx: GLint,
  position: GLuint,
  uv: GLuint,
  color: GLuint,
}

impl Renderer {
  pub fn new(
    imgui: &mut Context
  ) -> Self
  {
    let glock = GL.lock();
    let gl = glock.gl.as_ref().unwrap();

    unsafe {
      #[cfg(target_os = "macos")]
      let glsl_version = b"#version 150\n\0";
      #[cfg(not(target_os = "macos"))]
      let glsl_version = b"#version 130\n\0";

      let vert_source = b"
        uniform mat4 ProjMtx;
        in vec2 Position;
        in vec2 UV;
        in vec4 Color;
        out vec2 Frag_UV;
        out vec4 Frag_Color;
        void main()
        {
          Frag_UV = UV;
          Frag_Color = Color;
          gl_Position = ProjMtx * vec4(Position.xy,0,1);
        }
      \0";

      let frag_source = b"
        uniform sampler2D Texture;
        in vec2 Frag_UV;
        in vec4 Frag_Color;
        out vec4 Out_Color;
        void main()
        {
          Out_Color = Frag_Color * texture(Texture, Frag_UV.st);
        }
      \0";

      let vert_sources = [glsl_version.as_ptr() as *const GLchar,
                          vert_source.as_ptr() as *const GLchar];
      let vert_sources_len = [glsl_version.len() as GLint - 1,
                              vert_source.len() as GLint - 1];
      let frag_sources = [glsl_version.as_ptr() as *const GLchar,
                          frag_source.as_ptr() as *const GLchar];
      let frag_sources_len = [glsl_version.len() as GLint - 1,
                              frag_source.len() as GLint - 1];

      let program = gl.CreateProgram();
      let vert_shader = gl.CreateShader(VERTEX_SHADER);
      let frag_shader = gl.CreateShader(FRAGMENT_SHADER);
      gl.ShaderSource(vert_shader, 2, vert_sources.as_ptr(), vert_sources_len.as_ptr());
      gl.ShaderSource(frag_shader, 2, frag_sources.as_ptr(), frag_sources_len.as_ptr());
      gl.CompileShader(vert_shader);
      gl.CompileShader(frag_shader);
      gl.AttachShader(program, vert_shader);
      gl.AttachShader(program, frag_shader);
      gl.LinkProgram(program);
      gl.DeleteShader(vert_shader);
      gl.DeleteShader(frag_shader);

      let locs = Locs{
        texture: gl.GetUniformLocation(program, b"Texture\0".as_ptr() as _),
        proj_mtx: gl.GetUniformLocation(program, b"ProjMtx\0".as_ptr() as _),
        position: gl.GetAttribLocation(program, b"Position\0".as_ptr() as _) as _,
        uv: gl.GetAttribLocation(program, b"UV\0".as_ptr() as _) as _,
        color: gl.GetAttribLocation(program, b"Color\0".as_ptr() as _) as _,
      };

      let vbo = return_param(|x| gl.GenBuffers(1, x) );
      let ebo = return_param(|x| gl.GenBuffers(1, x) );

      let mut current_texture = 0;
      gl.GetIntegerv(TEXTURE_BINDING_2D, &mut current_texture);


      let font_texture = return_param(|x| gl.GenTextures(1, x));
      gl.BindTexture(TEXTURE_2D, font_texture);
      gl.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as _);
      gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);
      gl.PixelStorei(UNPACK_ROW_LENGTH, 0);

      {
        let mut atlas = imgui.fonts();

        let texture = atlas.build_rgba32_texture();
        gl.TexImage2D(TEXTURE_2D, 0, RGBA as _, texture.width as _, texture.height as _, 0, RGBA, UNSIGNED_BYTE, texture.data.as_ptr() as _);

        atlas.tex_id = (font_texture as usize).into();
      }

      gl.BindTexture(TEXTURE_2D, current_texture as _);

      Self{
        program,
        locs,
        vbo,
        ebo,
        font_texture,
      }
    }
  }

  pub fn render<'ui>(
    &self,
    ui: Ui<'ui>,
  ) {
    use imgui::{DrawVert,DrawIdx,DrawCmd,DrawCmdParams};

    let glock = GL.lock();
    let gl = glock.gl.as_ref().unwrap();

    unsafe {
      let last_active_texture = return_param(|x| gl.GetIntegerv(ACTIVE_TEXTURE, x));
      gl.ActiveTexture(TEXTURE0);
      let last_program = return_param(|x| gl.GetIntegerv(CURRENT_PROGRAM, x));
      let last_texture = return_param(|x| gl.GetIntegerv(TEXTURE_BINDING_2D, x));
      let last_sampler = if gl.BindSampler.is_loaded() { return_param(|x| gl.GetIntegerv(SAMPLER_BINDING, x)) } else { 0 };
      let last_array_buffer = return_param(|x| gl.GetIntegerv(ARRAY_BUFFER_BINDING, x));
      let last_element_array_buffer = return_param(|x| gl.GetIntegerv(ELEMENT_ARRAY_BUFFER_BINDING, x));
      let last_vertex_array = return_param(|x| gl.GetIntegerv(VERTEX_ARRAY_BINDING, x));
      //let last_polygon_mode = return_param(|x: &mut [GLint; 2]| gl.GetIntegerv(POLYGON_MODE, x.as_mut_ptr()));
      let last_viewport = return_param(|x: &mut [GLint; 4]| gl.GetIntegerv(VIEWPORT, x.as_mut_ptr()));
      let last_scissor_box = return_param(|x: &mut [GLint; 4]| gl.GetIntegerv(SCISSOR_BOX, x.as_mut_ptr()));
      let last_blend_src_rgb = return_param(|x| gl.GetIntegerv(BLEND_SRC_RGB, x));
      let last_blend_dst_rgb = return_param(|x| gl.GetIntegerv(BLEND_DST_RGB, x));
      let last_blend_src_alpha = return_param(|x| gl.GetIntegerv(BLEND_SRC_ALPHA, x));
      let last_blend_dst_alpha = return_param(|x| gl.GetIntegerv(BLEND_DST_ALPHA, x));
      let last_blend_equation_rgb = return_param(|x| gl.GetIntegerv(BLEND_EQUATION_RGB, x));
      let last_blend_equation_alpha = return_param(|x| gl.GetIntegerv(BLEND_EQUATION_ALPHA, x));
      let last_enable_blend = gl.IsEnabled(BLEND) == TRUE;
      let last_enable_cull_face = gl.IsEnabled(CULL_FACE) == TRUE;
      let last_enable_depth_test = gl.IsEnabled(DEPTH_TEST) == TRUE;
      let last_enable_scissor_test = gl.IsEnabled(SCISSOR_TEST) == TRUE;


      gl.Enable(BLEND);
      gl.BlendEquation(FUNC_ADD);
      gl.BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      gl.Disable(CULL_FACE);
      gl.Disable(DEPTH_TEST);
      gl.Enable(SCISSOR_TEST);
      //gl.PolygonMode(FRONT_AND_BACK, FILL);


      let [width, height] = ui.io().display_size;
      let [scale_w, scale_h] = ui.io().display_framebuffer_scale;

      let fb_width = width * scale_w;
      let fb_height = height * scale_h;

      gl.Viewport(0, 0, fb_width as _, fb_height as _);
      let matrix = [
        [ 2.0 / width as f32, 0.0,                     0.0, 0.0],
        [ 0.0,                2.0 / -(height as f32),  0.0, 0.0],
        [ 0.0,                0.0,                    -1.0, 0.0],
        [-1.0,                1.0,                     0.0, 1.0],
      ];
      gl.UseProgram(self.program);
      gl.Uniform1i(self.locs.texture, 0);
      gl.UniformMatrix4fv(self.locs.proj_mtx, 1, FALSE, matrix.as_ptr() as _);
      if gl.BindSampler.is_loaded() { gl.BindSampler(0, 0); }


      let vao = return_param(|x| gl.GenVertexArrays(1, x));
      gl.BindVertexArray(vao);
      gl.BindBuffer(ARRAY_BUFFER, self.vbo);
      gl.EnableVertexAttribArray(self.locs.position);
      gl.EnableVertexAttribArray(self.locs.uv);
      gl.EnableVertexAttribArray(self.locs.color);
      gl.VertexAttribPointer(self.locs.position, 2, FLOAT,         FALSE, mem::size_of::<DrawVert>() as _, field_offset::<DrawVert, _, _>(|v| &v.pos) as _);
      gl.VertexAttribPointer(self.locs.uv,       2, FLOAT,         FALSE, mem::size_of::<DrawVert>() as _, field_offset::<DrawVert, _, _>(|v| &v.uv) as _);
      gl.VertexAttribPointer(self.locs.color,    4, UNSIGNED_BYTE, TRUE,  mem::size_of::<DrawVert>() as _, field_offset::<DrawVert, _, _>(|v| &v.col) as _);


      let draw_data = ui.render();

      for draw_list in draw_data.draw_lists() {
        let vtx_buffer = draw_list.vtx_buffer();
        let idx_buffer = draw_list.idx_buffer();

        gl.BindBuffer(ARRAY_BUFFER, self.vbo);
        gl.BufferData(ARRAY_BUFFER, (vtx_buffer.len() * mem::size_of::<DrawVert>()) as _, vtx_buffer.as_ptr() as _, STREAM_DRAW);

        gl.BindBuffer(ELEMENT_ARRAY_BUFFER, self.ebo);
        gl.BufferData(ELEMENT_ARRAY_BUFFER, (idx_buffer.len() * mem::size_of::<DrawIdx>()) as _, idx_buffer.as_ptr() as _, STREAM_DRAW);

        for cmd in draw_list.commands() {
          match cmd {
            DrawCmd::Elements {
              count,
              cmd_params: DrawCmdParams {
                clip_rect: [x, y, z, w],
                texture_id,
                idx_offset,
                ..
              },
            } => {
              gl.BindTexture(TEXTURE_2D, texture_id.id() as _);

              gl.Scissor((x * scale_w) as GLint,
                         (fb_height - w * scale_h) as GLint,
                         ((z - x) * scale_w) as GLint,
                         ((w - y) * scale_h) as GLint);

              let idx_size = if mem::size_of::<DrawIdx>() == 2 { UNSIGNED_SHORT } else { UNSIGNED_INT };

              gl.DrawElements(TRIANGLES, count as _, idx_size, (idx_offset * mem::size_of::<DrawIdx>()) as _);
            },
            DrawCmd::ResetRenderState => {
              unimplemented!("Haven't implemented DrawCmd::ResetRenderState yet");
            },
            DrawCmd::RawCallback { .. } => {
              unimplemented!("Haven't implemented user callbacks yet");
            }
          }
        }
      }

      gl.DeleteVertexArrays(1, &vao);

      gl.UseProgram(last_program as _);
      gl.BindTexture(TEXTURE_2D, last_texture as _);
      if gl.BindSampler.is_loaded() { gl.BindSampler(0, last_sampler as _); }
      gl.ActiveTexture(last_active_texture as _);
      gl.BindVertexArray(last_vertex_array as _);
      gl.BindBuffer(ARRAY_BUFFER, last_array_buffer as _);
      gl.BindBuffer(ELEMENT_ARRAY_BUFFER, last_element_array_buffer as _);
      gl.BlendEquationSeparate(last_blend_equation_rgb as _, last_blend_equation_alpha as _);
      gl.BlendFuncSeparate(last_blend_src_rgb as _, last_blend_dst_rgb as _, last_blend_src_alpha as _, last_blend_dst_alpha as _);
      if last_enable_blend { gl.Enable(BLEND) } else { gl.Disable(BLEND) };
      if last_enable_cull_face { gl.Enable(CULL_FACE) } else { gl.Disable(CULL_FACE) };
      if last_enable_depth_test { gl.Enable(DEPTH_TEST) } else { gl.Disable(DEPTH_TEST) };
      if last_enable_scissor_test { gl.Enable(SCISSOR_TEST) } else { gl.Disable(SCISSOR_TEST) };
      //gl.PolygonMode(FRONT_AND_BACK, last_polygon_mode[0] as _);
      gl.Viewport(last_viewport[0] as _, last_viewport[1] as _, last_viewport[2] as _, last_viewport[3] as _);
      gl.Scissor(last_scissor_box[0] as _, last_scissor_box[1] as _, last_scissor_box[2] as _,  last_scissor_box[3] as _);

    }
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    let glock = GL.lock();
    let gl = glock.gl.as_ref().unwrap();

    unsafe {
      gl.DeleteBuffers(1, &self.vbo);
      gl.DeleteBuffers(1, &self.ebo);

      gl.DeleteProgram(self.program);

      gl.DeleteTextures(1, &self.font_texture);
    }
  }
}

fn field_offset<T, U, F: for<'a> FnOnce(&'a T) -> &'a U>(f: F) -> usize {
  unsafe {
    let instance = mem::zeroed::<T>();

    let offset = {
      let field: &U = f(&instance);
      field as *const U as usize - &instance as *const T as usize
    };

    mem::forget(instance);

    offset
  }
}

fn return_param<T, F>(f: F) -> T where F: FnOnce(&mut T) {
  let mut val = unsafe{ mem::zeroed() };
  f(&mut val);
  val
}