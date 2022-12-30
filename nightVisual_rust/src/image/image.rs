use super::pixels::{Pixels, Pixel};

// Goal 
//


// fn main() {


//     let buffer = ImageConfig::new()
//     .indicate_existing_colors(true)
//     .create_context()
//     .buffer(width, height, data)
//     .dark_mode();

// }


/// #ImgView
/// 
/// this trait is similar to `GenericImageView` trait provided by the `image` crate,
/// but contains only the necessary fields/methods for this crate.
trait ImgView {
    fn dimensions(&self) -> (u32, u32);
    fn get_pixel(&self, x: u32, y: u32) -> Pixel;
    fn pixels(&self) -> &Pixels;
}

pub struct ImageBuffer {
    width: u32,
    height: u32,
    data: Pixels //TODO
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
    /// Returns `Pixel`
    fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        // TODO: Return the Pixel at position (x,y)
        Pixel {
            // pixel_type: PixelType::Luma(1)
        }
    }

    /// Returns a iterator over the pixels of the image
    /// 
    /// Returns `Pixels` iterator
    fn pixels(&self) -> &Pixels {
        &self.data
    }
}

/// #ImageConfig
/// 
/// It offers various configurations to be used while performing
/// the dark mode operations on the given image buffer
pub struct ImageConfig {
    // TODO: Offer configuration options for flexibility
    indicate_existing_colors: bool
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
    pub fn new() -> Self {
        ImageConfig { indicate_existing_colors: false }
    }

    /// # ImageConfig::indicate_existing_colors()
    /// 
    /// Sets the indicate_existing_colors config
    /// 
    /// ## Arguments
    /// * `on` - bool - whether to on this config
    /// 
    /// ## Returns
    /// * `Self` - The ImageConfig instance on which this method is called
    pub fn indicate_existing_colors(mut self, on: bool) -> Self {
        self.indicate_existing_colors = on;
        self
    }

    pub fn create_image_context(self, buffer: ImageBuffer) -> ImageContext {
        ImageContext { config: self, buffer }
    }
}

/// # ImageContext
/// 
/// Provides all methods to perform various operations on the given image buffer
pub struct ImageContext {
    config: ImageConfig,
    buffer: ImageBuffer,
}

impl ImageContext {
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
