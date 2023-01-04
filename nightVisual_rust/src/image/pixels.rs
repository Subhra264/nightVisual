#[allow(dead_code)]
pub struct RgbaPixel {
  r: u8,
  g: u8,
  b: u8,
  a: u8
}

impl RgbaPixel {
  /// # RgbaPixel::new()
  /// 
  /// Creates a new RgbaPixel
  /// 
  /// ## Arguments
  /// * `r` - `u8` - The red value of the pixel
  /// * `g` - `u8` - The green value of the pixel
  /// * `b` - `u8` - The blue value of the pixel
  /// * `a` - `u8` - The a value of the pixel
  /// 
  /// ## Returns
  /// * `RgbaPixel` - A new RgbaPixel
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> RgbaPixel {
    RgbaPixel { r, g, b, a }
  }

  pub fn set_r(&mut self, r:u8) -> &Self {
    self.r = r;
    self
  }

  pub fn set_g(&mut self, g: u8) -> &Self {
    self.g = g;
    self
  }

}

/// # RgbPixel
/// 
/// Represents a Pixel with only `red`, `green` and `blue` values.
#[allow(dead_code)]
pub struct RgbPixel {
  r: u8,
  g: u8,
  b: u8
}

impl RgbPixel {

  /// # RgbPixel::new()
  /// 
  /// Creates a new RgbPixel
  /// 
  /// ## Arguments
  /// * `r` - `u8` - The `red` value of the pixel
  /// * `g` - `u8` - The `green` value of the pixel
  /// * `b` - `u8` - The `blue` value of the pixel
  /// 
  /// ## Returns
  /// * `RgbPixel` - Returns a new RgbPixel
  pub fn new(r: u8, g: u8, b:u8) -> RgbPixel {
    RgbPixel { r, g, b }
  }
}

/// # Pixel
/// 
/// An `enum` representing various types of Pixels
/// 
/// ## Values
/// * `RGBA(RgbaPixel)` - The `RgbaPixel` type that resolves with a `RgbaPixel`
/// * `RGB(RgbPixel)` - The `RgbPixel` type that resolves with a `RgbPixel`
pub enum Pixel {
  // TODO
  RGBA(RgbaPixel),
  RGB(RgbPixel),
  // Luma(u8)
}

// TODO: Implement Pixel methods generic to all Pixels
impl Pixel {
  // pub fn values(&self) -> &[u8] {
  //   match *self {
  //     Self::RGBA(rgba_pixel) => &[rgba_pixel.r, rgba_pixel.g, rgba_pixel.b, rgba_pixel.a],
  //     Self::RGB(rgb_pixel) => &[rgb_pixel.r, rgb_pixel.g, rgb_pixel.b]
  //   }
  // }
}