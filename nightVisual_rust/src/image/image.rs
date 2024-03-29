use super::pixels::{Pixel, RgbaPixel};
use std::fmt::Error;

// Goal
//
// fn main() {

//     let buffer = ImageConfig::new()
//     .indicate_existing_colors(true)
//     .create_context()
//     .buffer(width, height, data)
//     .dark_mode();

// ImageContext::new()
//     .create_buffer(width, height)
//     .fill_buffer(data)
//     .action(action)
//     .create_configs()
//     .indicate_existing_colors()

// ImageBuffer::new(width, height)
//     .data(vec![8,23,232,234,234])
//     .build_context()
//     .config()
//     .indicate_existing_colors()

// ImageBuffer::new(width, height)
// ImageBuffer::data(vec![])

// }

/// #ImgView
///
/// this trait is similar to `GenericImageView` trait provided by the `image` crate,
/// but contains only the necessary fields/methods for this crate.
pub trait ImgView {
    fn dimensions(&self) -> (u32, u32);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_pixel(&self, x: u32, y: u32) -> &Pixel;
    fn pixels(&self) -> &Vec<Pixel>;
}

pub struct ImageBuffer {
    width: u32,
    height: u32,
    data: Vec<Pixel>, //TODO
}

impl ImgView for ImageBuffer {
    /// Returns the dimensions of the image buffer
    ///
    /// Returns a tuple containing width and height sequentially.
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    /// Gives the pixel data in a certain position
    /// # Arguments
    /// * `x` - The x position of the pixel
    /// * `y` - The y position of the pixel
    ///
    /// Returns `&Pixel`
    fn get_pixel(&self, x: u32, y: u32) -> &Pixel {
        // Here, as x and y both are u32, they are always positive
        if x < self.width && y < self.height {
            // If `as usize` is not put, it will throw error
            // since, `(y * self.height + x)` is an u32 and u32 does not implement
            // the `SliceIndex<[Pixel]>` trait, use of usize is necessary as it
            // implements the `SliceIndex<[T]>` trait
            return &self.data[(y * self.height + x) as usize];
            // return Ok(&self.data[(y * self.height + x) as usize]);
        }

        panic!(
            "Image index {:?} out of bound {:?}",
            (x, y),
            (self.width, self.height)
        );
        // Err(Error(""))
    }

    /// Returns a iterator over the pixels of the image
    ///
    /// Returns `Pixels` iterator
    fn pixels(&self) -> &Vec<Pixel> {
        &self.data
    }
}

impl ImageBuffer {
    pub fn new_from_raw_data(width: u32, height: u32, data: &[u8]) -> ImageBuffer {
        let no_of_required_pixels = (width * height) as usize;

        if no_of_required_pixels == 0 {
            panic!("Width and Height must be non-zero!");
        }

        let mut pixels: Vec<Pixel> = Vec::new();

        let complete_pixels_in_data = data.len() / 4;
        let mut remaining_data = data.len() % 4;
        let no_of_complete_pixels = if no_of_required_pixels <= complete_pixels_in_data {
            no_of_required_pixels
        } else {
            complete_pixels_in_data
        };
        let mut no_of_remaining_pixels = no_of_required_pixels - no_of_complete_pixels;

        // For now lets consider only rgba images
        for i in 0..no_of_complete_pixels as usize {
            // (Width * Height) is the total number of pixels present in the image
            // Each pixel consists of 4 different values - r, g, b and a
            // Since, this method expects the data to be an one dimensional array of u8,
            // for each pixel( i.e. for each 'i'), we have to iterate over 4 consecutive elements
            let r = data[i * 4];
            let g = data[i * 4 + 1];
            let b = data[i * 4 + 2];
            let a = data[i * 4 + 3];

            pixels.push(Pixel::RGBA(RgbaPixel::new(r, g, b, a)));
        }

        if no_of_remaining_pixels > 0 {
            if remaining_data > 0 {
                let r = data[no_of_complete_pixels];
                let (mut g, mut b, a): (u8, u8, u8) = (255, 255, 255);
                remaining_data -= 1;

                if remaining_data > 0 {
                    g = data[no_of_complete_pixels + 1];
                    remaining_data -= 1;
                }

                if remaining_data > 0 {
                    b = data[no_of_complete_pixels + 2];
                    // remaining_data -= 1;
                }

                pixels.push(Pixel::RGBA(RgbaPixel::new(r, g, b, a)));
                no_of_remaining_pixels -= 1;
            }

            for _ in 0..no_of_remaining_pixels {
                pixels.push(Pixel::RGBA(RgbaPixel::new(255, 255, 255, 255)));
            }
        }

        ImageBuffer {
            width,
            height,
            data: pixels,
        }
    }

    pub fn new_from_pixels(width: u32, height: u32, data: Vec<Pixel>) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            data,
        }
    }
}

/// # DarkModePolicy
///
/// An enum representing the policy that is
/// going to be followed while applying dark mode
///
/// ## Possible values
///
/// * `INVERSE` - Inverse Policy that inverses the pixel colors
pub enum DarkModePolicy {
    INVERSE,
}

/// #ImageConfig
///
/// It offers various configurations to be used while performing
/// the dark mode operations on the given image buffer
pub struct ImageConfig {
    // TODO: Offer configuration options for flexibility
    indicate_existing_colors: bool,
    dark_mode_policy: DarkModePolicy,
}

// Methods to set the configurations
impl ImageConfig {
    /// # ImageConfig::new()
    ///
    /// Returns a new instance of ImageConfig struct with default configurations
    ///
    /// ## Default values of each configuration
    ///
    /// | Configurations | Default values |
    /// | -------------- | -------------- |
    /// | `indicate_existing_colors` | `false` |
    /// | `dark_mode_policy` | `INVERSE` |
    ///
    /// ## Returns
    /// * `ImageConfig` - a new `ImageConfig` instance
    pub fn new() -> ImageConfig {
        ImageConfig {
            indicate_existing_colors: false,
            dark_mode_policy: DarkModePolicy::INVERSE,
        }
    }

    /// # ImageConfig::set_indicate_existing_colors()
    ///
    /// Sets the indicate_existing_colors config
    ///
    /// ## Arguments
    /// * `on` - `bool` - whether to on this config
    ///
    /// ## Returns
    /// * `Self` - The ImageConfig instance on which this method is called
    pub fn set_indicate_existing_colors(mut self, on: bool) -> Self {
        self.indicate_existing_colors = on;
        self
    }

    /// # ImageConfig::set_dark_mode_policy()
    ///
    /// Sets the dark_mode_policy config
    ///
    /// ## Arguments
    /// * `policy` - `DarkModePolicy` - which policy to use while applying dark mode
    ///
    /// ## Returns
    /// * `Self` - The ImageConfig instance on which this method is called
    pub fn set_dark_mode_policy(mut self, policy: DarkModePolicy) -> Self {
        self.dark_mode_policy = policy;
        self
    }

    pub fn indicate_existing_colors(&self) -> bool {
        self.indicate_existing_colors
    }

    pub fn dark_mode_policy(&self) -> &DarkModePolicy {
        &self.dark_mode_policy
    }

    /// # ImageConfig::create_context()
    ///
    /// Creates a new ImageContext
    pub fn create_context(self) -> ImageContext {
        ImageContext::new(Some(self), None, None)
    }
}

// use std::ops::Fn;

// type ActionFunction = dyn Fn(Pixel, &ImageConfig) -> Pixel;
// type ActionFunction = Fn(u32, u16) -> u32;
type ActionFunction = fn(u32, u16) -> u32;

/// # ImageContext
///
/// Provides all methods to perform various operations on the given image buffer
pub struct ImageContext {
    config: ImageConfig,
    buffer: Option<ImageBuffer>,
    action: Option<ActionFunction>,
}

impl ImageContext {
    /// # ImageContext::new()
    ///
    /// Creates a new ImageContext instance
    ///
    /// ## Argumnents
    /// * `config` - `ImageConfig` - Required. All the configurations
    /// * `buffer` - `Option<ImageBuffer>` - Optional. The buffer on which the operations are to be performed
    /// * `action` - `Option<ActionFunction>` - Optional.The action function for custom algorithms
    ///
    /// ## Returns
    /// * `ImageContext` - Returns a new `ImageContext` instance
    pub fn new(
        config: Option<ImageConfig>,
        buffer: Option<ImageBuffer>,
        action: Option<ActionFunction>,
    ) -> ImageContext {
        let image_config: ImageConfig = match config {
            Some(given_options) => given_options,
            None => ImageConfig::new(),
        };

        ImageContext {
            config: image_config,
            buffer,
            action,
        }
    }

    /// # ImageContext::set_buffer()
    ///
    /// Sets the buffer of the ImageContext. This buffer will be used for dark mode operations
    ///
    /// ## Arguments
    /// * `buffer` - `ImageBuffer` - Required. The buffer to be used for dark mode
    ///
    /// ## Returns
    /// * `Self` - The `ImageContext` itself
    pub fn set_buffer(mut self, buffer: ImageBuffer) -> Self {
        self.buffer = Some(buffer);
        self
    }

    /// # ImageContext::set_config()
    ///
    /// Sets the configurations of the ImageContext. This config will be used for dark mode operations
    ///
    /// ## Arguments
    /// * `config` - `ImageConfig` - Required. The config to be used for dark mode
    ///
    /// ## Returns
    /// * `Self` - The `ImageContext` itself
    pub fn set_config(mut self, config: ImageConfig) -> Self {
        self.config = config;
        self
    }

    /// # ImageContext::set_action()
    ///
    /// Sets the action function of the ImageContext. This action will be used for custom algorithms
    ///
    /// ## Arguments
    /// * `action` - `ActionFunction` - Required.
    ///
    /// ## Returns
    /// * `Self` - The `ImageContext` itself
    pub fn set_action(mut self, action: ActionFunction) -> Self {
        self.action = Some(action);
        self
    }

    /// # ImageContext::is_buffer_set()
    ///
    /// Checks if a buffer is given or not
    ///
    /// ## Returns
    /// * `bool` - Returns `true` if buffer is set, `false` otherwise
    pub fn is_buffer_set(&self) -> bool {
        match self.buffer {
            Some(_) => true,
            None => false,
        }
    }

    /// # ImageContext::is_action_set()
    ///
    /// Checks if an action function is given
    ///
    /// ## Returns
    /// * `bool` - Returns `true` if action is set, `false` otherwise
    pub fn is_action_set(&self) -> bool {
        match self.action {
            Some(_) => true,
            None => false,
        }
    }

    /// # ImageContext::inverse_colors()
    ///
    /// Inverses the color pixels. Must be used after the buffer is set.
    ///
    /// ## Returns
    /// * `ImageBuffer` - A new ImageBuffer containing the inversed pixels
    fn inverse_colors(&self) -> ImageBuffer {
        //
        let buffer = self.buffer.as_ref().unwrap();
        let pixels = buffer.pixels();
        let (width, height) = buffer.dimensions();

        // let (width, height) = self.buffer?.dimensions();
        // let ref buffer_data = self.buffer?.data;
        // TODO: Inverse the pixels
        let mut inversed_pixels: Vec<Pixel> = Vec::new();

        for i in 0..(buffer.width() * buffer.height()) as usize {
            // For now, only dealing with RgbaPixels
            let ref pixel = pixels[i];

            match pixel {
                Pixel::RGBA(rgba_pixel) => {
                    let new_r = 255 - rgba_pixel.r();
                    let new_g = 255 - rgba_pixel.g();
                    let new_b = 255 - rgba_pixel.b();
                    let new_a = rgba_pixel.a();

                    // Let's not use a for now

                    inversed_pixels.push(Pixel::RGBA(RgbaPixel::new(new_r, new_g, new_b, new_a)));
                }
                _ => {} // TODO: Do nothing for now
            }
        }

        ImageBuffer::new_from_pixels(width, height, inversed_pixels)
    }

    /// ImageContext::apply_dark_mode()
    ///
    /// The heart of the this night_visual image library.
    /// Does the actual operations
    pub fn apply_dark_mode(&self) -> Result<ImageBuffer, Error> {
        if !self.is_buffer_set() {
            // Err(Error {})
            panic!("Buffer is not set!");
        }

        // match self.config.get_dark_mode_policy() {
        //     DarkModePolicy::INVERSE => Ok(self.inverse_colors())
        // }

        Ok(self.inverse_colors())
        // let newBuffer = ImageBuffer::new(self.width, self.height, [1,4]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const IMAGE_DATA: [u8; 8] = [25, 80, 61, 15, 89, 56, 156, 254];

    #[test]
    fn image_buffer_width_height_less_than_data_length() {
        let buffer = ImageBuffer::new_from_raw_data(1, 1, &IMAGE_DATA);

        let (width, height) = buffer.dimensions();

        assert_eq!(width, 1);
        assert_eq!(height, 1);
        assert_eq!(buffer.width(), 1);
        assert_eq!(buffer.height(), 1);
        assert_eq!(buffer.pixels().len(), 1);
    }

    #[test]
    fn image_buffer_width_height_match_with_data_length() {
        let buffer = ImageBuffer::new_from_raw_data(2, 1, &IMAGE_DATA);

        let (width, height) = buffer.dimensions();

        assert_eq!(width, 2);
        assert_eq!(height, 1);
        assert_eq!(buffer.width(), 2);
        assert_eq!(buffer.height(), 1);
        assert_eq!(buffer.pixels().len(), 2);
    }

    #[test]
    fn image_buffer_width_height_greater_than_data_length() {
        let buffer = ImageBuffer::new_from_raw_data(3, 2, &IMAGE_DATA);

        let (width, height) = buffer.dimensions();

        assert_eq!(width, 3);
        assert_eq!(height, 2);
        assert_eq!(buffer.width(), 3);
        assert_eq!(buffer.height(), 2);
        assert_eq!(buffer.pixels().len(), 6);
    }
}
