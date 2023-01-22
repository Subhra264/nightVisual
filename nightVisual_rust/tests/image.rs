use night_visual::image::image::*;
use night_visual::image::pixels::Pixel;

const IMAGE_DATA: [u8; 8] = [25, 80, 61, 15, 89, 56, 156, 254];

#[test]
fn correct_pixel_for_width_height_less_than_data_length() {
    let buffer = ImageBuffer::new_from_raw_data(1, 1, &IMAGE_DATA);
    assert_eq!(buffer.pixels().len(), 1);

    let pixel = buffer.get_pixel(0, 0);

    match pixel {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 25);
            assert_eq!(rgba_pixel.g(), 80);
            assert_eq!(rgba_pixel.b(), 61);
            assert_eq!(rgba_pixel.a(), 15);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }
}

#[test]
fn correct_pixel_for_width_height_equal_to_data_length() {
    let buffer = ImageBuffer::new_from_raw_data(2, 1, &IMAGE_DATA);
    assert_eq!(buffer.pixels().len(), 2);

    let pixel1 = buffer.get_pixel(0, 0);
    let pixel2 = buffer.get_pixel(1, 0);

    match pixel1 {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 25);
            assert_eq!(rgba_pixel.g(), 80);
            assert_eq!(rgba_pixel.b(), 61);
            assert_eq!(rgba_pixel.a(), 15);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }

    match pixel2 {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 89);
            assert_eq!(rgba_pixel.g(), 56);
            assert_eq!(rgba_pixel.b(), 156);
            assert_eq!(rgba_pixel.a(), 254);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }
}

#[test]
fn correct_pixel_for_width_height_greater_than_data_length() {
    let buffer = ImageBuffer::new_from_raw_data(2, 2, &IMAGE_DATA);
    assert_eq!(buffer.pixels().len(), 4);

    let pixel1 = buffer.get_pixel(0, 0);
    let pixel2 = buffer.get_pixel(1, 0);
    let pixel3 = buffer.get_pixel(0, 1);
    let pixel4 = buffer.get_pixel(1, 1);

    match pixel1 {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 25);
            assert_eq!(rgba_pixel.g(), 80);
            assert_eq!(rgba_pixel.b(), 61);
            assert_eq!(rgba_pixel.a(), 15);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }

    match pixel2 {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 89);
            assert_eq!(rgba_pixel.g(), 56);
            assert_eq!(rgba_pixel.b(), 156);
            assert_eq!(rgba_pixel.a(), 254);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }

    match pixel3 {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 255);
            assert_eq!(rgba_pixel.g(), 255);
            assert_eq!(rgba_pixel.b(), 255);
            assert_eq!(rgba_pixel.a(), 255);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }

    match pixel4 {
        Pixel::RGBA(rgba_pixel) => {
            assert_eq!(rgba_pixel.r(), 255);
            assert_eq!(rgba_pixel.g(), 255);
            assert_eq!(rgba_pixel.b(), 255);
            assert_eq!(rgba_pixel.a(), 255);
        }
        _ => assert!(false, "The pixel should be RGBA pixel"),
    }
}
