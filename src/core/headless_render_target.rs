use crate::context::{Context, Framebuffer, Renderbuffer};
use crate::core::*;

pub struct HeadlessTarget {
    pub width: usize,
    pub height: usize,
    pub framebuffer_id: Framebuffer,
    context: Context,
    renderbuffer_id: Renderbuffer,
}

impl HeadlessTarget {
    ///
    /// Constructs a new headless render target that enables rendering into an off-screen render buffer.
    ///
    pub fn new(
        context: &Context,
        pixel_format: Format,
        width: usize,
        height: usize,
    ) -> Result<Self, Error> {
        let cloned_context = context.clone();
        let (frame_buf, render_buf) =
            new_headless_target(&cloned_context, pixel_format, width, height)?;
        Ok(Self {
            context: cloned_context,
            width,
            height,
            framebuffer_id: frame_buf,
            renderbuffer_id: render_buf,
        })
    }

    pub fn resize_buffer(
        &self,
        pixel_format: Format,
        width: usize,
        height: usize,
    ) -> Result<(), Error> {
        return self.context.resize_renderbuffer_storage(
            self.renderbuffer_id,
            internal_format_from(pixel_format),
            width,
            height,
        );
    }
}

impl Drop for HeadlessTarget {
    fn drop(&mut self) {
        self.context
            .delete_headless_target(self.framebuffer_id, self.renderbuffer_id);
    }
}

fn new_headless_target(
    context: &Context,
    pixel_format: Format,
    width: usize,
    height: usize,
) -> Result<(Framebuffer, Renderbuffer), Error> {
    Ok(context
        .create_headless_buffers(internal_format_from(pixel_format), width, height)
        .ok_or_else(|| Error::HeadlessTargetError {
            message: "Failed to create headless target".to_string(),
        })?)
}
