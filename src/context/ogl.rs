pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::Error;
use std::rc::Rc;

use consts::Gl as InnerGl;

pub type AttributeLocation = u32;
pub type UniformLocation = u32;
pub type Shader = u32;
pub type Program = u32;
pub type Buffer = u32;
pub type Framebuffer = u32;
pub type Renderbuffer = u32;
pub type Texture = u32;
pub type VertexArrayObject = u32;
pub type Sync = consts::types::GLsync;
pub struct ActiveInfo {
    size: u32,
    type_: u32,
    name: String,
}
impl ActiveInfo {
    pub fn new(size: u32, type_: u32, name: String) -> ActiveInfo {
        ActiveInfo { size, type_, name }
    }
    pub fn size(&self) -> i32 {
        self.size as i32
    }
    pub fn type_(&self) -> u32 {
        self.type_
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

///
/// Contains the graphics API for almost direct calls to OpenGL/WebGL.
/// Used internally in the higher level features and can safely be ignored unless you want more control.
///
/// Calls to this API can be combined with higher level features.
///
#[derive(Clone)]
pub struct Context {
    // not used in stub: // inner: Rc<InnerGl>,
}

impl Context {
    pub fn new_stub() -> Context {
        Self {}
    }

    pub fn load_with<F>(loadfn: F) -> Context
    where
        for<'r> F: FnMut(&'r str) -> *const consts::types::GLvoid,
    {
        let gl = Context {
            // not used in stub: // inner: Rc::new(InnerGl::load_with(loadfn)),
        };
        gl.bind_vertex_array(&gl.create_vertex_array().unwrap());
        gl
    }

    pub fn finish(&self) {
        unsafe {
            /* not used in stub
;

             */
        }
    }

    pub fn create_shader(&self, type_: u32) -> Option<Shader> {
        let id = unsafe { /* not used in stub
self.inner.CreateShader(type_)
*/
            0
        };
        Some(id)
    }

    pub fn compile_shader(&self, source: &str, shader: &Shader) {
        let header = "#version 330 core\n";
        let s: &str = &[header, source].concat();

        use std::ffi::{CStr, CString};
        let c_str: &CStr = &CString::new(s).unwrap();

        unsafe {
            /* not used in stub
;
            self.inner.CompileShader(*shader);

             */
        }
    }

    pub fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        let mut len: consts::types::GLint = 0;
        unsafe {
            /* not used in stub
self.inner
                .GetShaderiv(*shader, consts::INFO_LOG_LENGTH, &mut len);

             */
        }

        if len == 0 {
            None
        } else {
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                /* not used in stub
self.inner.GetShaderInfoLog(
                    *shader,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut consts::types::GLchar,
                );

                 */
            }
            Some(error.to_string_lossy().into_owned())
        }
    }

    pub fn delete_shader(&self, shader: Option<&Shader>) {
        unsafe {
            /* not used in stub
self.inner.DeleteShader(*shader.unwrap());

             */
        }
    }

    pub fn attach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            /* not used in stub
self.inner.AttachShader(*program, *shader);

             */
        }
    }

    pub fn detach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            /* not used in stub
self.inner.DetachShader(*program, *shader);

             */
        }
    }

    pub fn get_program_parameter(&self, program: &Program, pname: u32) -> u32 {
        let mut out = 0;
        unsafe {
        }
        out as u32
    }

    pub fn get_active_attrib(&self, program: &Program, index: u32) -> ActiveInfo {
        let mut length = 128;
        let mut size = 0;
        let mut _type = 0;
        let name = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            ;
        }

        let mut s = name.to_string_lossy().into_owned();
        s.truncate(length as usize);
        ActiveInfo::new(size as u32, _type as u32, s)
    }

    pub fn get_active_uniform(&self, program: &Program, index: u32) -> ActiveInfo {
        let mut length = 128;
        let mut size = 0;
        let mut _type = 0;
        let name = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            ;
        }

        let mut s = name.to_string_lossy().into_owned();
        s.truncate(length as usize);
        ActiveInfo::new(size as u32, _type as u32, s)
    }

    pub fn create_buffer(&self) -> Option<Buffer> {
        let mut id: u32 = 0;
        unsafe {
            ;
        }
        Some(id)
    }

    pub fn delete_buffer(&self, buffer: &Buffer) {
        unsafe {
            ;
        }
    }

    pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: &Buffer) {
        let pname = match target {
            consts::ARRAY_BUFFER => consts::ARRAY_BUFFER_BINDING,
            consts::ELEMENT_ARRAY_BUFFER => consts::ELEMENT_ARRAY_BUFFER_BINDING,
            consts::UNIFORM_BUFFER => consts::UNIFORM_BUFFER_BINDING,
            _ => unreachable!(),
        };

        unsafe {
            let mut current = -1;
            ;
            if current != 0 {
                println!("{}", current);
                panic!();
            }
            ;
        }
    }

    pub fn bind_buffer(&self, target: u32, buffer: &Buffer) {
        unsafe {
            ;
        }
    }

    pub fn unbind_buffer(&self, target: u32) {
        unsafe {
            ;
        }
    }

    pub fn get_uniform_block_index(&self, program: &Program, name: &str) -> u32 {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            0
        }
    }

    pub fn uniform_block_binding(&self, program: &Program, location: u32, index: u32) {
        unsafe {
        }
    }

    pub fn buffer_data(&self, target: u32, size_in_bytes: u32, usage: u32) {
        unsafe {
            ;
        }
    }

    pub fn buffer_data_u8(&self, target: u32, data: &[u8], usage: u32) {
        unsafe {
            ;
        }
    }

    pub fn buffer_data_u32(&self, target: u32, data: &[u32], usage: u32) {
        unsafe {
            ;
        }
    }

    pub fn buffer_data_f32(&self, target: u32, data: &[f32], usage: u32) {
        unsafe {
            ;
        }
    }

    pub fn create_vertex_array(&self) -> Option<VertexArrayObject> {
        let mut id: u32 = 0;
        unsafe {
            ;
        }
        Some(id)
    }

    pub fn bind_vertex_array(&self, array: &VertexArrayObject) {
        unsafe {
            ;
        }
    }

    pub fn create_program(&self) -> Program {
        unsafe {
            0
        }
    }

    pub fn link_program(&self, program: &Program) -> bool {
        unsafe {
        }

        let mut success: consts::types::GLint = 1;
        unsafe {
            ;
        }
        return true;
    }

    pub fn get_program_info_log(&self, program: &Program) -> Option<String> {
        let mut len: consts::types::GLint = 0;
        unsafe {
            ;
        }

        if len == 0 {
            None
        } else {
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                ;
            }
            Some(error.to_string_lossy().into_owned())
        }
    }

    pub fn use_program(&self, program: &Program) {
        unsafe {
            ;
        }
    }

    pub fn unuse_program(&self) {
        unsafe {
            ;
        }
    }

    pub fn delete_program(&self, program: &Program) {
        unsafe {
            ;
        }
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Option<AttributeLocation> {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe { 0 };
        if location == -1 {
            None
        } else {
            Some(location as AttributeLocation)
        }
    }

    pub fn enable_vertex_attrib_array(&self, location: AttributeLocation) {
        unsafe {
            ;
        }
    }

    pub fn disable_vertex_attrib_array(&self, location: AttributeLocation) {
        unsafe {
            ;
        }
    }

    pub fn vertex_attrib_pointer(
        &self,
        location: AttributeLocation,
        size: u32,
        data_type: u32,
        normalized: bool,
        stride: u32,
        offset: u32,
    ) {
        unsafe {
            ;
        }
    }

    pub fn vertex_attrib_divisor(&self, location: AttributeLocation, divisor: u32) {
        unsafe {
            ;
        }
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str) -> Option<UniformLocation> {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe { 0 };
        if location == -1 {
            None
        } else {
            Some(location as UniformLocation)
        }
    }

    pub fn uniform1i(&self, location: &UniformLocation, data: i32) {
        unsafe {
            ;
        }
    }

    pub fn uniform1f(&self, location: &UniformLocation, data: f32) {
        unsafe {
            ;
        }
    }

    pub fn uniform2fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            ;
        }
    }

    pub fn uniform3fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            ;
        }
    }

    pub fn uniform4fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            ;
        }
    }

    pub fn uniform_matrix2fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            ;
        }
    }

    pub fn uniform_matrix3fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            ;
        }
    }

    pub fn uniform_matrix4fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            ;
        }
    }

    pub fn draw_buffers(&self, draw_buffers: &[u32]) {
        unsafe {
            ;
        }
    }

    pub fn create_framebuffer(&self) -> Option<Framebuffer> {
        let mut id: u32 = 0;
        unsafe {
            ;
        }
        Some(id)
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<&Framebuffer>) {
        let id = match framebuffer {
            Some(fb) => *fb,
            None => 0,
        };
        unsafe {
            ;
        }
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&Framebuffer>) {
        let id = match framebuffer {
            Some(fb) => fb,
            None => &0,
        };
        unsafe {
            ;
        }
    }

    pub fn check_framebuffer_status(&self) -> Result<(), String> {
        let status = unsafe { consts::FRAMEBUFFER_COMPLETE };

        match status {
            consts::FRAMEBUFFER_COMPLETE => Ok(()),
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                Err("FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
                Err("FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                Err("FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string())
            }
            consts::FRAMEBUFFER_UNSUPPORTED => Err("FRAMEBUFFER_UNSUPPORTED".to_string()),
            consts::FRAMEBUFFER_UNDEFINED => Err("FRAMEBUFFER_UNDEFINED".to_string()),
            consts::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
                Err("FRAMEBUFFER_INCOMPLETE_READ_BUFFER".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                Err("FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                Err("FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS".to_string())
            }
            _ => Err("Unknown framebuffer error".to_string()),
        }
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: u32,
        src_y0: u32,
        src_x1: u32,
        src_y1: u32,
        dst_x0: u32,
        dst_y0: u32,
        dst_x1: u32,
        dst_y1: u32,
        mask: u32,
        filter: u32,
    ) {
        unsafe {
            ;
        }
    }

    pub fn create_headless_buffers(
        &self,
        pixel_format: u32,
        width: usize,
        height: usize,
    ) -> Option<(crate::context::Framebuffer, crate::context::Renderbuffer)> {
        // inspired by https://github.com/rust-windowing/glutin/blob/bab33a84dfb094ff65c059400bed7993434638e2/glutin_examples/examples/headless.rs#L89-L113
        let mut fb = 0;
        let mut render_buf = 0;
        unsafe {
            // Using the fb backing a pbuffer is very much a bad idea. Fails on
            // many platforms, and is deprecated. Better just make your own fb.
            //
            // Surfaceless doesn't come with a surface, as the name implies, so
            // you must make your own fb.
            //
            // Making an fb is not neccesary with osmesa, however, can't be bothered
            // to have a different code path.
            ;
            self.resize_renderbuffer_storage(render_buf, pixel_format, width, height)
                .ok()?;
            ;
            ;
            ;

        }
        Some((fb, render_buf))
    }

    pub fn resize_renderbuffer_storage(
        &self,
        render_buf_id: u32,
        pixel_format: u32,
        width: usize,
        height: usize,
    ) -> Result<(), Error> {
        unsafe {
            ;
            ;
        }
        return Ok(());
    }

    pub fn delete_headless_target(
        &self,
        framebuffer_id: crate::context::Framebuffer,
        renderbuffer_id: crate::context::Renderbuffer,
    ) {
        unsafe {
            ;
            ;
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: usize, height: usize) {
        unsafe {
            ;
        }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            ;
        }
    }

    pub fn clear_depth(&self, depth: f32) {
        unsafe {
            ;
        }
    }

    pub fn clear(&self, mask: u32) {
        unsafe {
            ;
        }
    }

    pub fn enable(&self, cap: u32) {
        unsafe {
            ;
        }
    }

    pub fn disable(&self, cap: u32) {
        unsafe {
            ;
        }
    }

    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
        unsafe {
            ;
        }
    }

    pub fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
        unsafe {
            ;
        }
    }

    pub fn blend_equation(&self, mode: u32) {
        unsafe {
            ;
        }
    }

    pub fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        unsafe {
            ;
        }
    }

    pub fn cull_face(&self, mode: u32) {
        unsafe {
            ;
        }
    }

    pub fn depth_func(&self, func: u32) {
        unsafe {
            ;
        }
    }

    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        unsafe {
            ;
        }
    }

    pub fn depth_mask(&self, flag: bool) {
        unsafe {
            if flag {
                ;
            } else {
                ;
            }
        }
    }

    pub fn create_texture(&self) -> Option<Texture> {
        let mut id: u32 = 0;
        unsafe {
            ;
        }
        Some(id)
    }

    pub fn active_texture(&self, texture: u32) {
        unsafe {
            ;
        }
    }

    pub fn bind_texture(&self, target: u32, texture: &Texture) {
        unsafe {
            ;
        }
    }

    pub fn generate_mipmap(&self, target: u32) {
        unsafe {
            ;
        }
    }

    pub fn tex_storage_2d(
        &self,
        target: u32,
        levels: u32,
        internalformat: u32,
        width: u32,
        height: u32,
    ) {
        unsafe {
            ;
        }
    }

    pub fn tex_storage_3d(
        &self,
        target: u32,
        levels: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            ;
        }
    }

    pub fn tex_image_2d(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        border: u32,
        format: u32,
        data_type: u32,
    ) {
        unsafe {
            ;
        }
    }

    pub fn tex_image_2d_with_u8_data(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        border: u32,
        format: u32,
        data_type: u32,
        pixels: &[u8],
    ) {
        unsafe {
            ;
        }
    }

    pub fn tex_sub_image_2d_with_u8_data(
        &self,
        target: u32,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: u32,
        pixels: &[u8],
    ) {
        unsafe {
        }
    }

    pub fn tex_image_2d_with_f32_data(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        border: u32,
        format: u32,
        data_type: u32,
        pixels: &[f32],
    ) {
        unsafe {
            /* not used in stub
            ;
            */
        }
    }

    pub fn tex_sub_image_2d_with_f32_data(
        &self,
        target: u32,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: u32,
        pixels: &[f32],
    ) {
        unsafe {
            /* not used in stub
            ;
            */
        }
    }

    pub fn tex_image_3d(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
        format: u32,
        data_type: u32,
    ) {
        unsafe {
            /* not used in stub
            ;
            */
        }
    }

    pub fn tex_image_3d_with_u16_data(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
        border: u32,
        format: u32,
        data_type: u32,
        pixels: &[u16],
    ) {
        unsafe {
            /* not used in stub
            ;
            */
        }
    }

    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        unsafe {
            // not used in stub // ;
        }
    }

    pub fn delete_texture(&self, texture: &Texture) {
        unsafe {
        }
    }

    pub fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: &Texture,
        level: u32,
    ) {
        unsafe {
        }
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: &Texture,
        level: u32,
        layer: u32,
    ) {
        unsafe {
        }
    }

    pub fn draw_arrays(&self, mode: u32, first: u32, count: u32) {
        unsafe {
        }
    }

pub fn draw_arrays_instanced(&self, mode: u32, first: u32, count: u32, instance_count: u32) {
    unsafe {
    }
}

pub fn draw_elements(&self, mode: u32, count: u32, data_type: u32, offset: u32) {
    unsafe {
        /* // not used in stub
        ;
        */
    }
}

pub fn draw_elements_instanced(
    &self,
    mode: u32,
    count: u32,
    data_type: u32,
    offset: u32,
    instance_count: u32,
) {
    unsafe {
        /* // not used in stub
        ;
        */
    }
}

    pub fn read_pixels_with_f32_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: u32,
        dst_data: &mut [f32],
    ) {
        unsafe {}
    }

    pub fn read_pixels_with_u16_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: u32,
        dst_data: &mut [u16],
    ) {
        unsafe {

        }
    }

pub fn read_pixels(
    &self,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    format: u32,
    data_type: u32,
) {
    unsafe {
        /* // not used in stub
        self.inner.ReadPixels(
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            format,
            data_type,
            0 as *mut consts::types::GLvoid,
        );
        */
    }
}

pub fn read_pixels_with_u8_data(
    &self,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    format: u32,
    data_type: u32,
    dst_data: &mut [u8],
) {
    unsafe {
        /* // not used in stub
        ;
        self.inner.ReadPixels(
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            format,
            data_type,
            dst_data.as_mut_ptr() as *mut consts::types::GLvoid,
        )
        */
    }
}

pub fn flush(&self) {
    unsafe {
        /* // not used in stub
        self.inner.Flush();
        */
    }
}

pub fn fence_sync(&self) -> Sync {
    unsafe {
        /* // not used in stub
        self.inner.FenceSync(consts::SYNC_GPU_COMMANDS_COMPLETE, 0)
         */
        return std::ptr::null();
}
}

pub fn client_wait_sync(&self, sync: &Sync, flags: u32, timeout: u32) -> u32 {
unsafe {
    /* // not used in stub
    self.inner.ClientWaitSync(*sync, flags, timeout as u64)
    */
    return 0;
}
}

pub fn delete_sync(&self, sync: &Sync) {
unsafe {
    /* // not used in stub
    ;
    */
}
}
}

fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
// allocate buffer of correct size
let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
// fill it with len spaces
buffer.extend([b' '].iter().cycle().take(len));
// convert buffer to CString
unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}

fn byte_size_for_type(data_type: u32, count: u32) -> u32 {
match data_type {
    consts::FLOAT => count * std::mem::size_of::<f32>() as u32,
    consts::UNSIGNED_INT => count * std::mem::size_of::<u32>() as u32,
    _ => 0,
}
}
