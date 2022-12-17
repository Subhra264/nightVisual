use super::pixels::{Pixels, Pixel, PixelType};

trait ImgView<> {
    fn dimensions(&self) -> (u32, u32);
    fn get_pixel(&self, x: u32, y: u32) -> Pixel;
    fn pixels(&self) -> Pixels<PixelType>;
}

pub struct ImageBuffer {
    width: u32,
    height: u32,
    data: Pixels<PixelType> //TODO
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

    }

    /// Returns a iterator over the pixels of the image
    /// 
    /// Returns `Pixels` iterator
    fn pixels(&self) -> Pixels<PixelType> {
        self.data
    }
}

struct Buffer {
    width: u32,
    height: u32,
    container: Pixels<PixelType>
}

impl Buffer {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        // TODO
    }

    fn pixels(&self) -> Pixels<PixelType> {
        self.container
    }
}

pub struct Image {
    pub title: String,
    pub size: usize,
    data: dyn ImgView,
}

impl Image {
    pub fn create_image(size: usize) -> Image {
        let buffer = Buffer {
            width: size,
            height: size,
            container: {}
        };

        Self::create_image_from_buffer(buffer);

        Image {
            title: String::from("image.jpg"),
            size,
            data: ImageBuffer::new(size, size),
        }
    }

    pub fn create_image_from_buffer(img: dyn ImgView) -> Image {
        // Image {
        //     title
        // }
    }
}
