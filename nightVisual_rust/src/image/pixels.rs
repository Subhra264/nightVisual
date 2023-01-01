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
}

pub enum Pixel {
  // TODO
  RGBA(RgbaPixel),
  // RGB(f32),
  // Luma(u8)
}