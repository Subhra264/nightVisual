use std::fmt::Error;
use super::pixels::{Pixel, RgbaPixel};

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
trait ImgView {
    fn dimensions(&self) -> (u32, u32);
    fn get_pixel(&self, x: u32, y: u32) -> &Pixel;
    fn pixels(&self) -> &Vec<Pixel>;
}

pub struct ImageBuffer {
    width: u32,
    height: u32,
    data: Vec<Pixel> //TODO
}

impl ImgView for ImageBuffer {

    /// Returns the dimensions of the image buffer
    ///
    /// Returns a tuple containing width and height sequentially.
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
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
        }
        
        panic!("Image index {:?} out of bound {:?}", (x, y), (self.width, self.height));
    }

    /// Returns a iterator over the pixels of the image
    ///
    /// Returns `Pixels` iterator
    fn pixels(&self) -> &Vec<Pixel> {
        &self.data
    }
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32, data: &[u8]) -> ImageBuffer {
        let mut pixels: Vec<Pixel> = Vec::new();

        // For now lets consider only rgba images
        for i in 0..(width * height) as usize {

            // (Width * Height) is the total number of pixels present in the image
            // Each pixel consists of 4 different values - r, g, b and a
            // Since, this method expects the data to be an one dimensional array of u8,
            // for each pixel( i.e. for each 'i'), we have to iterate over 4 consecutive elements
            let r = data[i*4];
            let g = data[i*4 + 1];
            let b = data[i*4 + 2];
            let a = data[i*4 + 3];

            pixels.push(Pixel::RGBA(RgbaPixel::new(r, g, b, a)));
        }

        ImageBuffer { width, height, data: pixels }
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
    dark_mode_policy: DarkModePolicy
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
            dark_mode_policy: DarkModePolicy::INVERSE
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

    pub fn get_dark_mode_policy(self) -> DarkModePolicy {
        self.dark_mode_policy
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
    action: Option<ActionFunction>
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
    pub fn new(config: Option<ImageConfig>, buffer: Option<ImageBuffer>, action: Option<ActionFunction>) -> ImageContext {
        let image_config: ImageConfig = match config {
            Some(given_options) => given_options,
            None => ImageConfig::new()
        };
        
        ImageContext {
            config: image_config,
            buffer,
            action
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
            None => false
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
            None => false
        }
    }

    /// # ImageContext::inverse_colors()
    /// 
    /// inverses the color pixels
    /// 
    /// ## Returns
    /// * `ImageBuffer` - A new ImageBuffer containing the inversed pixels
    fn inverse_colors(&self) -> ImageBuffer {
        // TODO: Inverse the pixels
        ImageBuffer::new(1, 1, &[1,2])
    }

    /// ImageContext::apply_dark_mode()
    /// 
    /// The heart of the this night_visual image library.
    /// Does the actual 
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
    // pub fn create_image(width: u32, height: u32) {
    //     // let buffer = Buffer {
    //     //     width,
    //     //     height,
    //     //     container: vec![1,2,3,4]
    //     // };

    //     let buffer = ImageBuffer {
    //         width,
    //         height,
    //         data: Pixels {
    //             // pixels: [Pixel {}]
    //         }
    //     };

    //     // Self::create_image_from_buffer(&buffer as &dyn ImgView);
    //     // Self::create_image_from_buffer(&buffer as &ImageBuffer);
    //     Self::create_image_from_buffer(&buffer as &Buffer);

    //     // Image {
    //     //     title: String::from("image.jpg"),
    //     //     size,
    //     //     data: ImageBuffer::new(size, size),
    //     // }
    // }

    // // pub fn create_image_from_buffer(img: &dyn ImgView) {
    // pub fn create_image_from_buffer(img: &Buffer) {
    //     // Image {
    //     //     title
    //     // }
    //     let _image = img;
        
    // }
}
