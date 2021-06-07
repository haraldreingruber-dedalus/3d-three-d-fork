use crate::definition::*;
use crate::io::*;
use std::path::Path;

impl<'a> Loaded<'a> {
    ///
    /// Deserialize the loaded image resource at the given path into a [CPUTexture](crate::CPUTexture) using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [Texture2D](crate::Texture2D).
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn image<P: AsRef<Path>>(&'a self, path: P) -> Result<CPUTexture<u8>, IOError> {
        use image::GenericImageView;
        let img = image::load_from_memory(self.bytes(path)?)?;
        let bytes = img.to_bytes();
        let number_of_channels = bytes.len() / (img.width() * img.height()) as usize;
        let format = match number_of_channels {
            1 => Ok(Format::R8),
            3 => Ok(Format::RGB8),
            4 => Ok(Format::RGBA8),
            _ => Err(IOError::FailedToLoad {
                message: format!("Could not determine the pixel format for the texture."),
            }),
        }?;

        Ok(CPUTexture {
            data: bytes,
            width: img.width() as usize,
            height: img.height() as usize,
            format,
            ..Default::default()
        })
    }

    ///
    /// Deserialize the 6 loaded image resources at the given paths into a [CPUTexture](crate::CPUTexture) using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [TextureCubeMap](crate::TextureCubeMap).
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn cube_image<P: AsRef<Path>>(
        &'a self,
        right_path: P,
        left_path: P,
        top_path: P,
        bottom_path: P,
        front_path: P,
        back_path: P,
    ) -> Result<CPUTexture<u8>, IOError> {
        let mut right = self.image(right_path)?;
        let left = self.image(left_path)?;
        let top = self.image(top_path)?;
        let bottom = self.image(bottom_path)?;
        let front = self.image(front_path)?;
        let back = self.image(back_path)?;

        right.data.extend(left.data);
        right.data.extend(top.data);
        right.data.extend(bottom.data);
        right.data.extend(front.data);
        right.data.extend(back.data);
        Ok(right)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {
    ///
    /// Saves the given RGB pixels as an image.
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn save_pixels<P: AsRef<Path>>(
        path: P,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) -> Result<(), IOError> {
        let pixels_out = Saver::convert_from_opengl_coordinates::<u8>(pixels, width, height, 4);

        image::save_buffer(
            path,
            pixels_out.as_slice(),
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )?;
        Ok(())
    }

    pub fn save_pixels_16bit_grayscale<P: AsRef<Path>>(
        path: P,
        pixels: &[u16],
        width: usize,
        height: usize,
    ) -> Result<(), IOError> {
        let pixels_out = Saver::convert_from_opengl_coordinates::<u16>(pixels, width, height, 1);

        let img = image::ImageBuffer::<image::Luma<u16> ,&[u16]>::from_raw(width as u32, height as u32, pixels_out.as_slice()).unwrap();
        img.save(path)?;
        Ok(())
    }

    fn convert_from_opengl_coordinates<T: Default + Copy + Clone>(pixels: &[T], width: usize, height: usize, channels: usize) -> Vec<T> {
        let mut pixels_out = vec![T::default(); width * height * channels];
        for row in 0..height {
            for col in 0..width {
                for i in 0..channels {
                    pixels_out[channels * width * (height - row - 1) + channels * col + i] =
                        pixels[channels * width * row + channels * col + i];
                }
            }
        }
        pixels_out
    }
}
